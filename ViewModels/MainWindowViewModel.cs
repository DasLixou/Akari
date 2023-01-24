using ReactiveUI;
using System.Collections.ObjectModel;

namespace Akari.ViewModels;

public class MainWindowViewModel : ViewModelBase
{
    private int selectedIndex;

    public int SelectedIndex
    {
        get => selectedIndex;
        set => this.RaiseAndSetIfChanged(ref selectedIndex, value);
    }

    public ObservableCollection<object> Pages { get; set; } = new();

    public MainWindowViewModel()
    {
        Pages.Add("scribe");
        Pages.Add("books");
        Pages.Add("calender");
        Pages.Add("cook");
        Pages.Add("settings");
    }
}