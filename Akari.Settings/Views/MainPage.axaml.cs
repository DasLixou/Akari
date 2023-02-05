using Avalonia.Controls;
using Avalonia.Markup.Xaml;

namespace Akari.Settings.Views
{
    public partial class MainPage : UserControl
    {
        public MainPage()
        {
            InitializeComponent();
        }

        private void InitializeComponent()
        {
            AvaloniaXamlLoader.Load(this);
        }
    }
}