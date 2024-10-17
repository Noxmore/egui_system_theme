import SwiftUI
import AppKit

func colors(color: Colors) -> (Double, Double, Double, Double) {
    let color: [CGFloat] = switch color {
    case Colors.Text:
        NSColor.textColor.cgColor.components!
    case Colors.Link:
        NSColor.linkColor.cgColor.components!
    case Colors.Black:
        NSColor(Color.black).cgColor.components!
    case Colors.Red:
        NSColor.systemRed.cgColor.components!
    case Colors.White:
        NSColor(Color.white).cgColor.components!
    case Colors.Clear:
        NSColor(Color.clear).cgColor.components!
    case Colors.Blue:
        NSColor.systemBlue.cgColor.components!
    case Colors.Gray:
        NSColor.systemGray.cgColor.components!
    case Colors.Green:
        NSColor.systemGreen.cgColor.components!
    case Colors.Primary:
        NSColor(Color.primary).cgColor.components!
    case Colors.Accent:
        NSColor(Color.accentColor).cgColor.components!
    case Colors.Secondary:
        NSColor(Color.secondary).cgColor.components!
    case Colors.Yellow:
        NSColor.systemYellow.cgColor.components!
    case Colors.Brown:
        NSColor.systemBrown.cgColor.components!
    case Colors.Cyan:
        NSColor.systemCyan.cgColor.components!
    case Colors.Indigo:
        NSColor.systemIndigo.cgColor.components!
    case Colors.Mint:
        NSColor.systemMint.cgColor.components!
    case Colors.Orange:
        NSColor.systemOrange.cgColor.components!
    case Colors.Pink:
        NSColor.systemPink.cgColor.components!
    case Colors.Purple:
        NSColor.systemPurple.cgColor.components!
    case Colors.Teal:
        NSColor.systemTeal.cgColor.components!
    case Colors.Separator:
        NSColor.separatorColor.cgColor.components!
    case Colors.TextEdit:
        NSColor.textBackgroundColor.cgColor.components!
    case Colors.Shadow:
        NSColor.shadowColor.cgColor.components!
    case Colors.InputCursor:
        NSColor.textInsertionPointColor.cgColor.components!
    case Colors.Window:
        NSColor.windowBackgroundColor.cgColor.components!
    case Colors.InactiveFg:
        NSColor.knobColor.cgColor.components!
    case Colors.Stripe:
        NSColor.alternatingContentBackgroundColors[0].cgColor.components!
    case Colors.ScrollBar:
        NSColor.scrollBarColor.cgColor.components!
    }
    
    return (color[0], color[1], color[2], color[3])
}
