using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Diagnostics;
using System.Drawing;
using System.Drawing.Imaging;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading;
using System.Windows.Forms;

namespace BerryIndicator
{
    public partial class BerryForm : Form
    {
        string tmpImage;

        public BerryForm()
        {
            InitializeComponent();
            foreach (Control control in Controls)
            {
                control.Anchor ^= AnchorStyles.Top | AnchorStyles.Bottom;
            }
            BerryIndicatorImg.Anchor = AnchorStyles.Top | AnchorStyles.Left;
            int gutterWidth = Size.Width - ClientSize.Width;
            Size = new Size(Math.Max(Width, 480) + gutterWidth, Height + 340);
            BerryIndicatorImg.Size = new Size(480, 340);
            BoxBackground.SelectedIndex = 0;
            tmpImage = Path.ChangeExtension(Path.GetTempFileName(), "bmp");
        }

        object updateLock = new object();
        bool updating = false;
        private void UpdateImage()
        {
            lock (updateLock)
            {
                if (updating)
                    return;
                updating = true;
            }

            ImageParams imgParams = new ImageParams();
            imgParams.saveFile = TxtSaveFile.Text;
            imgParams.tmpFile = tmpImage;
            imgParams.background = BoxBackground.Text;
            imgParams.addSpacing = ChkSpacing.Checked;
            imgParams.showDeaths = ChkDeaths.Checked;
            imgParams.showHearts = ChkHearts.Checked;
            imgParams.showSpoilers = ChkSpoilers.Checked;

            Thread t = new Thread(new ParameterizedThreadStart(UpdateHandler));
            t.Start(imgParams);
        }

        private void UpdateHandler(object obj)
        {
            ImageParams imgParams = obj as ImageParams;
            if (imgParams == null || !File.Exists(imgParams.saveFile))
            {
                lock (updateLock)
                    updating = false;
                Invoke(new ImageUpdateDelegate(OnImageUpdated), new object[] { null });
                return;
            }

            StringBuilder args = new StringBuilder();
            args.AppendFormat("\"{0}\" \"{1}\" /bg={2}", imgParams.saveFile, imgParams.tmpFile, imgParams.background);
            if (imgParams.showDeaths)
                args.Append(" /deaths");
            if (!imgParams.showHearts)
                args.Append(" /no-hearts");
            if (!imgParams.showSpoilers)
                args.Append(" /no-spoilers");
            if (imgParams.addSpacing)
                args.Append(" /spacing");

            ProcessStartInfo info = new ProcessStartInfo("celeste-berry-indicator-win64.exe", args.ToString());
            info.CreateNoWindow = true;
            info.RedirectStandardOutput = true;
            info.UseShellExecute = false;
            Process process = Process.Start(info);
            process.WaitForExit();

            FileStream read = new FileStream(imgParams.tmpFile, FileMode.Open, FileAccess.Read, FileShare.Read);
            Bitmap img = new Bitmap(read);
            read.Dispose();

            lock (updateLock)
                updating = false;

            Invoke(new ImageUpdateDelegate(OnImageUpdated), img);
        }

        delegate void ImageUpdateDelegate(object obj);

        private void OnImageUpdated(object obj)
        {
            Bitmap image = obj as Bitmap;
            Image currImage = BerryIndicatorImg.Image;
            BerryIndicatorImg.Image = image;
            if (currImage != null)
                currImage.Dispose();
            LblClickToSave.Visible = BerryIndicatorImg.Image != null;
        }

        private void BerryIndicatorImg_Click(object sender, EventArgs e)
        {
            if (BerryIndicatorImg.Image == null)
                return;

            using (SaveFileDialog dialog = new SaveFileDialog())
            {
                dialog.Filter = "Portable Network Graphics|*.png|Bitmap Image File|*.bmp|Graphics Interchange Format|*.gif|Joint Photographic Experts Group|*.jpeg";
                if (dialog.ShowDialog() == System.Windows.Forms.DialogResult.OK)
                {
                    Image img = BerryIndicatorImg.Image;
                    Bitmap imageClone = new Bitmap(img.Width, img.Height);
                    using (Graphics g = Graphics.FromImage(imageClone))
                        g.DrawImage(img, 0, 0);
                    string extension = Path.GetExtension(dialog.FileName);
                    try
                    {
                        ImageFormat format = ExtensionToFormat(extension);
                    }
                    catch (ArgumentException)
                    {
                        MessageBox.Show(String.Format("Unsupported extension \"{0}\".", extension),
                            "Unknown Filetype", MessageBoxButtons.OK, MessageBoxIcon.Error);
                    }
                    using (FileStream write = new FileStream(dialog.FileName, FileMode.Create, FileAccess.Write, FileShare.None))
                    {
                        imageClone.Save(write, ImageFormat.Png);
                    }
                }
            }
        }

        private void BtnSelectSaveFile_Click(object sender, EventArgs e)
        {
            using (OpenFileDialog dialog = new OpenFileDialog())
            {
                if (dialog.ShowDialog() == System.Windows.Forms.DialogResult.OK)
                {
                    TxtSaveFile.Text = dialog.FileName;
                    uint id = 0;
                    if (UInt32.TryParse(Path.GetFileNameWithoutExtension(dialog.FileName), out id))
                        NumSaveID.Enabled = true;
                    else
                    {
                        NumSaveID.Enabled = false;
                        id = 0;
                    }
                    suppressNumUpdate = true;
                    NumSaveID.Value = id;
                    suppressNumUpdate = false;
                    UpdateImage();
                }
            }
        }

        bool suppressNumUpdate;
        private void NumSaveID_ValueChanged(object sender, EventArgs e)
        {
            if (suppressNumUpdate)
                return;

            TxtSaveFile.Text = Path.Combine(Path.GetDirectoryName(TxtSaveFile.Text), NumSaveID.Value + Path.GetExtension(TxtSaveFile.Text));
            UpdateImage();
        }

        private void ChkSpoilers_CheckedChanged(object sender, EventArgs e)
        {
            UpdateImage();
        }

        private void ChkHearts_CheckedChanged(object sender, EventArgs e)
        {
            UpdateImage();
        }

        private void ChkDeaths_CheckedChanged(object sender, EventArgs e)
        {
            UpdateImage();
        }

        private void ChkSpacing_CheckedChanged(object sender, EventArgs e)
        {
            UpdateImage();
        }

        private void BoxBackground_SelectedIndexChanged(object sender, EventArgs e)
        {
            UpdateImage();
        }

        private ImageFormat ExtensionToFormat(string extension)
        {
            extension = extension.ToLowerInvariant();
            switch (extension)
            {
                case ".bmp":
                    return ImageFormat.Bmp;
                case ".gif":
                    return ImageFormat.Gif;
                case ".jpg":
                case ".jpeg":
                    return ImageFormat.Jpeg;
                case ".png":
                    return ImageFormat.Png;
                case ".tiff":
                    return ImageFormat.Tiff;
                default:
                    throw new ArgumentException();
            }
        }
    }
}
