using Akari.Settings.ViewModels;
using Avalonia.Controls;
using Avalonia.Markup.Xaml;

namespace Akari.Settings.Views
{
    public partial class MainPage : UserControl
    {
        public MainPage()
        {
            DataContext = new MainPageViewModel();
            InitializeComponent();
        }

        private void InitializeComponent()
        {
            AvaloniaXamlLoader.Load(this);
        }
    }
}