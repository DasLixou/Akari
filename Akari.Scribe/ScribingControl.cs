using Avalonia;
using Avalonia.Controls;
using Avalonia.Input;
using Avalonia.Media;

namespace Akari.Scribe;

public class ScribingControl : Control
{
    private List<Point> points = new(100000);

    protected override void OnPointerMoved(PointerEventArgs e)
    {
        base.OnPointerMoved(e);
        if (e.InputModifiers.HasFlag(InputModifiers.LeftMouseButton))
        {
            Console.WriteLine("MÖVE");
            points.Add(e.GetPosition(this));
            this.InvalidateVisual();
        }
    }

    public override void Render(DrawingContext context)
    {
        base.Render(context);
        context.DrawRectangle(Brushes.Black, new Pen(Brushes.Black), new Rect(0, 0, this.Bounds.Width, this.Bounds.Height));
        foreach (var p in points)
        {
            context.DrawRectangle(Brushes.Wheat, new Pen(Brushes.Wheat), new Rect(p, new Size(1, 1)));
        }
    }
}