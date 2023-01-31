using Avalonia;
using Avalonia.Controls;
using Avalonia.Input;
using Avalonia.Media;

namespace Akari.Scribe;

public class ScribingControl : Control
{
    private PathGeometry geometry = new();
    private PathFigure currentFigure = null;

    protected override void OnPointerPressed(PointerPressedEventArgs e)
    {
        base.OnPointerPressed(e);
        var figure = new PathFigure() { StartPoint = e.GetPosition(this), IsClosed = true, IsFilled = false };
        geometry.Figures.Add(figure);
        currentFigure = figure;
    }

    protected override void OnPointerMoved(PointerEventArgs e)
    {
        base.OnPointerMoved(e);
        if (e.InputModifiers.HasFlag(InputModifiers.LeftMouseButton))
        {
            Console.WriteLine("MÖVE");
            var segment = new LineSegment() { Point = e.GetPosition(this) };
            currentFigure.Segments.Add(segment);
            var figure = new PathFigure() { StartPoint = e.GetPosition(this), IsClosed = true, IsFilled = false };
            geometry.Figures.Add(figure);
            currentFigure = figure;
            this.InvalidateVisual();
        }
    }

    protected override void OnPointerReleased(PointerReleasedEventArgs e)
    {
        base.OnPointerReleased(e);
        var segment = new LineSegment() { Point = e.GetPosition(this) };
        currentFigure.Segments.Add(segment);
        currentFigure = null;
    }

    public override void Render(DrawingContext context)
    {
        base.Render(context);
        context.DrawRectangle(Brushes.Black, new Pen(Brushes.Black), new Rect(0, 0, this.Bounds.Width, this.Bounds.Height));
        context.DrawGeometry(Brushes.Bisque, new Pen(Brushes.Bisque), geometry);
    }
}