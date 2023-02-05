using Akari.Models;
using Akari.ViewModels;
using Akari.Views;
using AuroraModularis;
using AuroraModularis.Core;
using Avalonia;
using Avalonia.Controls.ApplicationLifetimes;
using Avalonia.Markup.Xaml;

namespace Akari
{
    public partial class App : Application
    {
        public override void Initialize()
        {
            AvaloniaXamlLoader.Load(this);
        }

        public override async void OnFrameworkInitializationCompleted()
        {
            var bootstrapper = BootstrapperBuilder.StartConfigure()
                .WithAppName("Akari")
                .WithModulesBasePath(".")
                .WithSettingsBasePath(".");

            await bootstrapper.BuildAndStartAsync();

            ServiceContainer.Current.Resolve<ISidebarService>().Initialize();

            if (ApplicationLifetime is IClassicDesktopStyleApplicationLifetime desktop)
            {
                desktop.MainWindow = new MainWindow
                {
                    DataContext = new MainWindowViewModel(),
                };
            }

            base.OnFrameworkInitializationCompleted();
        }
    }
}