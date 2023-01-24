using Akari.Commands;
using Akari.Models;
using AuroraModularis.Core;
using ReactiveUI;
using System.Collections.ObjectModel;
using System.Linq;

namespace Akari.ViewModels;

public class MainWindowViewModel : ViewModelBase
{
    private int selectedIndex;

    public string First { get => Container.Current.Resolve<ISidebarService>().Elements.First().Title; }

    public int SelectedIndex
    {
        get => selectedIndex;
        set => this.RaiseAndSetIfChanged(ref selectedIndex, value);
    }

    public ObservableCollection<object> Pages { get; set; } = new();

    public SwitchPageCommand SwitchPage { get; set; }

    public MainWindowViewModel()
    {
        SwitchPage = new(this);
        Pages.Add("scribe");
        Pages.Add("books");
        Pages.Add("calender");
        Pages.Add("cook");
        Pages.Add("settings");
    }
}