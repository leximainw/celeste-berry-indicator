namespace BerryIndicator
{
    partial class BerryForm
    {
        /// <summary>
        /// Required designer variable.
        /// </summary>
        private System.ComponentModel.IContainer components = null;

        /// <summary>
        /// Clean up any resources being used.
        /// </summary>
        /// <param name="disposing">true if managed resources should be disposed; otherwise, false.</param>
        protected override void Dispose(bool disposing)
        {
            if (disposing && (components != null))
            {
                components.Dispose();
            }
            base.Dispose(disposing);
        }

        #region Windows Form Designer generated code

        /// <summary>
        /// Required method for Designer support - do not modify
        /// the contents of this method with the code editor.
        /// </summary>
        private void InitializeComponent()
        {
            System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(BerryForm));
            this.BerryIndicatorImg = new System.Windows.Forms.PictureBox();
            this.LblSaveFile = new System.Windows.Forms.Label();
            this.TxtSaveFile = new System.Windows.Forms.TextBox();
            this.BtnSelectSaveFile = new System.Windows.Forms.Button();
            this.LblSaveID = new System.Windows.Forms.Label();
            this.NumSaveID = new System.Windows.Forms.NumericUpDown();
            this.ChkSpoilers = new System.Windows.Forms.CheckBox();
            this.ChkHearts = new System.Windows.Forms.CheckBox();
            this.ChkDeaths = new System.Windows.Forms.CheckBox();
            this.LblClickToSave = new System.Windows.Forms.Label();
            ((System.ComponentModel.ISupportInitialize)(this.BerryIndicatorImg)).BeginInit();
            ((System.ComponentModel.ISupportInitialize)(this.NumSaveID)).BeginInit();
            this.SuspendLayout();
            // 
            // BerryIndicatorImg
            // 
            this.BerryIndicatorImg.Location = new System.Drawing.Point(0, 0);
            this.BerryIndicatorImg.Name = "BerryIndicatorImg";
            this.BerryIndicatorImg.Size = new System.Drawing.Size(12, 12);
            this.BerryIndicatorImg.TabIndex = 0;
            this.BerryIndicatorImg.TabStop = false;
            this.BerryIndicatorImg.Click += new System.EventHandler(this.BerryIndicatorImg_Click);
            // 
            // LblSaveFile
            // 
            this.LblSaveFile.AutoSize = true;
            this.LblSaveFile.Location = new System.Drawing.Point(12, 28);
            this.LblSaveFile.Name = "LblSaveFile";
            this.LblSaveFile.Size = new System.Drawing.Size(91, 13);
            this.LblSaveFile.TabIndex = 1;
            this.LblSaveFile.Text = "Save file location:";
            // 
            // TxtSaveFile
            // 
            this.TxtSaveFile.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.TxtSaveFile.Location = new System.Drawing.Point(109, 25);
            this.TxtSaveFile.Name = "TxtSaveFile";
            this.TxtSaveFile.Size = new System.Drawing.Size(97, 20);
            this.TxtSaveFile.TabIndex = 2;
            // 
            // BtnSelectSaveFile
            // 
            this.BtnSelectSaveFile.Anchor = ((System.Windows.Forms.AnchorStyles)((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right)));
            this.BtnSelectSaveFile.Location = new System.Drawing.Point(212, 23);
            this.BtnSelectSaveFile.Name = "BtnSelectSaveFile";
            this.BtnSelectSaveFile.Size = new System.Drawing.Size(60, 23);
            this.BtnSelectSaveFile.TabIndex = 3;
            this.BtnSelectSaveFile.Text = "Select";
            this.BtnSelectSaveFile.UseVisualStyleBackColor = true;
            this.BtnSelectSaveFile.Click += new System.EventHandler(this.BtnSelectSaveFile_Click);
            // 
            // LblSaveID
            // 
            this.LblSaveID.AutoSize = true;
            this.LblSaveID.Location = new System.Drawing.Point(12, 53);
            this.LblSaveID.Name = "LblSaveID";
            this.LblSaveID.Size = new System.Drawing.Size(65, 13);
            this.LblSaveID.TabIndex = 4;
            this.LblSaveID.Text = "Save file ID:";
            // 
            // NumSaveID
            // 
            this.NumSaveID.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.NumSaveID.Location = new System.Drawing.Point(109, 51);
            this.NumSaveID.Maximum = new decimal(new int[] {
            1000000,
            0,
            0,
            0});
            this.NumSaveID.Name = "NumSaveID";
            this.NumSaveID.Size = new System.Drawing.Size(97, 20);
            this.NumSaveID.TabIndex = 5;
            this.NumSaveID.ValueChanged += new System.EventHandler(this.NumSaveID_ValueChanged);
            // 
            // ChkSpoilers
            // 
            this.ChkSpoilers.AutoSize = true;
            this.ChkSpoilers.Checked = true;
            this.ChkSpoilers.CheckState = System.Windows.Forms.CheckState.Checked;
            this.ChkSpoilers.Location = new System.Drawing.Point(12, 77);
            this.ChkSpoilers.Name = "ChkSpoilers";
            this.ChkSpoilers.Size = new System.Drawing.Size(93, 17);
            this.ChkSpoilers.TabIndex = 6;
            this.ChkSpoilers.Text = "Show Spoilers";
            this.ChkSpoilers.UseVisualStyleBackColor = true;
            this.ChkSpoilers.CheckedChanged += new System.EventHandler(this.ChkSpoilers_CheckedChanged);
            // 
            // ChkHearts
            // 
            this.ChkHearts.AutoSize = true;
            this.ChkHearts.Checked = true;
            this.ChkHearts.CheckState = System.Windows.Forms.CheckState.Checked;
            this.ChkHearts.Location = new System.Drawing.Point(109, 77);
            this.ChkHearts.Name = "ChkHearts";
            this.ChkHearts.Size = new System.Drawing.Size(87, 17);
            this.ChkHearts.TabIndex = 7;
            this.ChkHearts.Text = "Show Hearts";
            this.ChkHearts.UseVisualStyleBackColor = true;
            this.ChkHearts.CheckedChanged += new System.EventHandler(this.ChkHearts_CheckedChanged);
            // 
            // ChkDeaths
            // 
            this.ChkDeaths.AutoSize = true;
            this.ChkDeaths.Location = new System.Drawing.Point(202, 77);
            this.ChkDeaths.Name = "ChkDeaths";
            this.ChkDeaths.Size = new System.Drawing.Size(90, 17);
            this.ChkDeaths.TabIndex = 8;
            this.ChkDeaths.Text = "Show Deaths";
            this.ChkDeaths.UseVisualStyleBackColor = true;
            this.ChkDeaths.CheckedChanged += new System.EventHandler(this.ChkDeaths_CheckedChanged);
            // 
            // LblClickToSave
            // 
            this.LblClickToSave.AutoSize = true;
            this.LblClickToSave.Font = new System.Drawing.Font("Microsoft Sans Serif", 8.25F, System.Drawing.FontStyle.Italic, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.LblClickToSave.Location = new System.Drawing.Point(36, 9);
            this.LblClickToSave.Name = "LblClickToSave";
            this.LblClickToSave.Size = new System.Drawing.Size(114, 13);
            this.LblClickToSave.TabIndex = 9;
            this.LblClickToSave.Text = "Click on image to save";
            this.LblClickToSave.Visible = false;
            // 
            // Form1
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(6F, 13F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.ClientSize = new System.Drawing.Size(284, 106);
            this.Controls.Add(this.LblClickToSave);
            this.Controls.Add(this.ChkDeaths);
            this.Controls.Add(this.ChkHearts);
            this.Controls.Add(this.ChkSpoilers);
            this.Controls.Add(this.NumSaveID);
            this.Controls.Add(this.LblSaveID);
            this.Controls.Add(this.BtnSelectSaveFile);
            this.Controls.Add(this.TxtSaveFile);
            this.Controls.Add(this.LblSaveFile);
            this.Controls.Add(this.BerryIndicatorImg);
            this.Icon = ((System.Drawing.Icon)(resources.GetObject("$this.Icon")));
            this.Name = "Form1";
            this.Text = "Celeste Berry Indicator";
            ((System.ComponentModel.ISupportInitialize)(this.BerryIndicatorImg)).EndInit();
            ((System.ComponentModel.ISupportInitialize)(this.NumSaveID)).EndInit();
            this.ResumeLayout(false);
            this.PerformLayout();

        }

        #endregion

        private System.Windows.Forms.PictureBox BerryIndicatorImg;
        private System.Windows.Forms.Label LblSaveFile;
        private System.Windows.Forms.TextBox TxtSaveFile;
        private System.Windows.Forms.Button BtnSelectSaveFile;
        private System.Windows.Forms.Label LblSaveID;
        private System.Windows.Forms.NumericUpDown NumSaveID;
        private System.Windows.Forms.CheckBox ChkSpoilers;
        private System.Windows.Forms.CheckBox ChkHearts;
        private System.Windows.Forms.CheckBox ChkDeaths;
        private System.Windows.Forms.Label LblClickToSave;
    }
}

