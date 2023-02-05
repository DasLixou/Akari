using AuroraModularis.Core;

namespace Akari.Scribe;

[Priority]
public class Module : AuroraModularis.Module
{
    public override Task OnStart(ServiceContainer container)
    {
        return Task.CompletedTask;
    }
}