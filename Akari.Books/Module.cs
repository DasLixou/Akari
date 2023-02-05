﻿using AuroraModularis.Core;

namespace Akari.Books;

[Priority]
public class Module : AuroraModularis.Module
{
    public override Task OnStart(ServiceContainer container)
    {
        return Task.CompletedTask;
    }
}