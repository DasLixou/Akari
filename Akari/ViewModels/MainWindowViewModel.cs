using Akari.Models;
using AuroraModularis.Core;
using DynamicData;
using System.Collections.ObjectModel;

namespace Akari.ViewModels;

public class MainWindowViewModel : ViewModelBase
{
    public ObservableCollection<object> Pages { get; set; } = new();

    public MainWindowViewModel()
    {
        Pages.AddRange(Container.Current.Resolve<ISidebarService>().Elements);
    }
}