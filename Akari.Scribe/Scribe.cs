﻿using Akari.Models;
using Akari.Scribe.Views;
using Avalonia.Controls;

namespace Akari.Scribe;

public class Scribe : ISidebarElement
{
    public string Title => "Scribe";

    public string OutlinedIcon => "Assets/Brush.svg";

    public UserControl Content => new MainPage();
}