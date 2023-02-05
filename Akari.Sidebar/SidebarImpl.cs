using Akari.Models;
using AuroraModularis.Core;

namespace Akari.Sidebar;

public class SidebarImpl : ISidebarService
{
    public IReadOnlyList<ISidebarElement> PrimaryElements { get; set; }
    public IReadOnlyList<ISidebarElement> SecondaryElements { get; set; }

    public void Initialize()
    {
        var prim_elements = new List<ISidebarElement>();
        var secnd_elements = new List<ISidebarElement>();
        Console.WriteLine("Load Sidebar applications... une moment, s'il vous plaît...");
        foreach (var ele in ServiceContainer.Current.Resolve<ITypeFinder>().FindAndResolveTypes<ISidebarElement>().OrderBy(e => e.Priority))
        {
            Console.WriteLine("Loading {0}...", ele.Title);
            if (ele.IsPrimary)
            {
                prim_elements.Add(ele);
            }
            else
            {
                secnd_elements.Add(ele);
            }
        }
        PrimaryElements = prim_elements;
        SecondaryElements = secnd_elements;
    }
}