using Akari.Models;
using AuroraModularis.Core;
using ReactiveUI;
using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;

namespace Akari.ViewModels;

public class MainWindowViewModel : ViewModelBase
{
    public ObservableCollection<object> PrimaryPages { get; set; }
    public ObservableCollection<object> SecondaryPages { get; set; }

    public IReadOnlyList<object> Pages { get; set; }

    private int _pageIndex;

    public int PageIndex
    {
        get => _pageIndex;
        set
        {
            Console.WriteLine("'" + value + "'");
            this.RaiseAndSetIfChanged(ref _pageIndex, value);
        }
    }

    public int SecPageIndex
    {
        get => _pageIndex - PrimaryPages.Count;
        set
        {
            Console.WriteLine("'" + value + "'");
            this.RaiseAndSetIfChanged(ref _pageIndex, value + PrimaryPages.Count);
        }
    }

    public MainWindowViewModel()
    {
        var sidebar_service = ServiceContainer.Current.Resolve<ISidebarService>();
        PrimaryPages = new(sidebar_service.PrimaryElements);
        SecondaryPages = new(sidebar_service.SecondaryElements);

        Pages = PrimaryPages.Concat(SecondaryPages).ToList();
    }
}