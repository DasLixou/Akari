using Akari.Models;
using AuroraModularis.Core;

namespace Akari.Sidebar;

[Priority]
public class Module : AuroraModularis.Module
{
    public override Task OnStart(Container container)
    {
        return Task.CompletedTask;
    }

    public override void RegisterServices(Container container)
    {
        container.Register<ISidebarService>(new SidebarImpl()).AsSingleton();
    }
}