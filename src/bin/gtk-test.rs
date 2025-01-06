use fontmanager::font_manager::{FontInfo, FontManager, FontStyle};
use gtk4::{glib, Application, ApplicationWindow, DrawingArea};
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, DrawingAreaExt, DrawingAreaExtManual, GtkWindowExt, WidgetExt};
use pangocairo::functions::{create_layout, show_layout};
use pangocairo::pango;

const APP_ID: &str = "io.gosub.font-manager.gtk-test";


const TEXT: &str = r"§ 1 Of the Nature of Flatland

I call our world Flatland, not because we call it so, but to make its nature clearer to you, my happy readers, who are privileged to live in Space.

Imagine a vast sheet of paper on which straight Lines, Triangles, Squares, Pentagons, Hexagons, and other figures, instead of remaining fixed in their places, move freely about, on or in the surface, but without the power of rising above or sinking below it, very much like shadows—only hard with luminous edges—and you will then have a pretty correct notion of my country and countrymen. Alas, a few years ago, I should have said “my universe:” but now my mind has been opened to higher views of things.

In such a country, you will perceive at once that it is impossible that there should be anything of what you call a “solid” kind; but I dare say you will suppose that we could at least distinguish by sight the Triangles, Squares, and other figures, moving about as I have described them. On the contrary, we could see nothing of the kind, not at least so as to distinguish one figure from another. Nothing was visible, nor could be visible, to us, except Straight Lines; and the necessity of this I will speedily demonstrate.

Place a penny on the middle of one of your tables in Space; and leaning over it, look down upon it. It will appear a circle.

But now, drawing back to the edge of the table, gradually lower your eye (thus bringing yourself more and more into the condition of the inhabitants of Flatland), and you will find the penny becoming more and more oval to your view, and at last when you have placed your eye exactly on the edge of the table (so that you are, as it were, actually a Flatlander) the penny will then have ceased to appear oval at all, and will have become, so far as you can see, a straight line.

The same thing would happen if you were to treat in the same way a Triangle, or a Square, or any other figure cut out from pasteboard. As soon as you look at it with your eye on the edge of the table, you will find that it ceases to appear to you as a figure, and that it becomes in appearance a straight line. Take for example an equilateral Triangle—who represents with us a Tradesman of the respectable class. Figure 1 represents the Tradesman as you would see him while you were bending over him from above; figures 2 and 3 represent the Tradesman, as you would see him if your eye were close to the level, or all but on the level of the table; and if your eye were quite on the level of the table (and that is how we see him in Flatland) you would see nothing but a straight line.

 When I was in Spaceland I heard that your sailors have very similar experiences while they traverse your seas and discern some distant island or coast lying on the horizon. The far-off land may have bays, forelands, angles in and out to any number and extent; yet at a distance you see none of these (unless indeed your sun shines bright upon them revealing the projections and retirements by means of light and shade), nothing but a grey unbroken line upon the water.

Well, that is just what we see when one of our triangular or other acquaintances comes towards us in Flatland. As there is neither sun with us, nor any light of such a kind as to make shadows, we have none of the helps to the sight that you have in Spaceland. If our friend comes closer to us we see his line becomes larger; if he leaves us it becomes smaller; but still he looks like a straight line; be he a Triangle, Square, Pentagon, Hexagon, Circle, what you will—a straight Line he looks and nothing else.

You may perhaps ask how under these disadvantagous circumstances we are able to distinguish our friends from one another: but the answer to this very natural question will be more fitly and easily given when I come to describe the inhabitants of Flatland. For the present let me defer this subject, and say a word or two about the climate and houses in our country.

§ 2 Of the Climate and Houses in Flatland

As with you, so also with us, there are four points of the compass North, South, East, and West.

There being no sun nor other heavenly bodies, it is impossible for us to determine the North in the usual way; but we have a method of our own. By a Law of Nature with us, there is a constant attraction to the South; and, although in temperate climates this is very slight—so that even a Woman in reasonable health can journey several furlongs northward without much difficulty—yet the hampering effort of the southward attraction is quite sufficient to serve as a compass in most parts of our earth. Moreover, the rain (which falls at stated intervals) coming always from the North, is an additional assistance; and in the towns we have the guidance of the houses, which of course have their side-walls running for the most part North and South, so that the roofs may keep off the rain from the North. In the country, where there are no houses, the trunks of the trees serve as some sort of guide. Altogether, we have not so much difficulty as might be expected in determining our bearings.

Yet in our more temperate regions, in which the southward attraction is hardly felt, walking sometimes in a perfectly desolate plain where there have been no houses nor trees to guide me, I have been occasionally compelled to remain stationary for hours together, waiting till the rain came before continuing my journey. On the weak and aged, and especially on delicate Females, the force of attraction tells much more heavily than on the robust of the Male Sex, so that it is a point of breeding, if you meet a Lady on the street, always to give her the North side of the way—by no means an easy thing to do always at short notice when you are in rude health and in a climate where it is difficult to tell your North from your South.

Windows there are none in our houses: for the light comes to us alike in our homes and out of them, by day and by night, equally at all times and in all places, whence we know not. It was in old days, with our learned men, an interesting and oft-investigate question, “What is the origin of light?” and the solution of it has been repeatedly attempted, with no other result than to crowd our lunatic asylums with the would-be solvers. Hence, after fruitless attempts to suppress such investigations indirectly by making them liable to a heavy tax, the Legislature, in comparatively recent times, absolutely prohibited them. I—alas, I alone in Flatland—know now only too well the true solution of this mysterious problem; but my knowledge cannot be made intelligible to a single one of my countrymen; and I am mocked at—I, the sole possessor of the truths of Space and of the theory of the introduction of Light from the world of three Dimensions—as if I were the maddest of the mad! But a truce to these painful digressions: let me return to our homes.

The most common form for the construction of a house is five-sided or pentagonal, as in the annexed figure. The two Northern sides RO, OF, constitute the roof, and for the most part have no doors; on the East is a small door for the Women; on the West a much larger one for the Men; the South side or floor is usually doorless.

 Square and triangular houses are not allowed, and for this reason. The angles of a Square (and still more those of an equilateral Triangle,) being much more pointed than those of a Pentagon, and the lines of inanimate objects (such as houses) being dimmer than the lines of Men and Women, it follows that there is no little danger lest the points of a square or triangular house residence might do serious injury to an inconsiderate or perhaps absentminded traveller suddenly running against them: and therefore, as early as the eleventh century of our era, triangular houses were universally forbidden by Law, the only exceptions being fortifications, powder-magazines, barracks, and other state buildings, which is not desirable that the general public should approach without circumspection.

At this period, square houses were still everywhere permitted, though discouraged by a special tax. But, about three centuries afterwards, the Law decided that in all towns containing a population above ten thousand, the angle of a Pentagon was the smallest house-angle that could be allowed consistently with the public safety. The good sense of the community has seconded the efforts of the Legislature; and now, even in the country, the pentagonal construction has superseded every other. It is only now and then in some very remote and backward agricultural district that an antiquarian may still discover a square house.

§ 3 Concerning the Inhabitants of Flatland

The greatest length or breadth of a full grown inhabitant of Flatland may be estimated at about eleven of your inches. Twelve inches may be regarded as a maximum.

Our Women are Straight Lines.

Our Soldiers and Lowest Class of Workmen are Triangles with two equal sides, each about eleven inches long, and a base or third side so short (often not exceeding half an inch) that they form at their vertices a very sharp and formidable angle. Indeed when their bases are of the most degraded type (not more than the eighth part of an inch in size), they can hardly be distinguished from Straight lines or Women; so extremely pointed are their vertices. With us, as with you, these Triangles are distinguished from others by being called Isosceles; and by this name I shall refer to them in the following pages.

Our Middle Class consists of Equilateral or Equal-Sided Triangles.

Our Professional Men and Gentlemen are Squares (to which class I myself belong) and Five-Sided Figures or Pentagons.

Next above these come the Nobility, of whom there are several degrees, beginning at Six-Sided Figures, or Hexagons, and from thence rising in the number of their sides till they receive the honourable title of Polygonal, or many-Sided. Finally when the number of the sides becomes so numerous, and the sides themselves so small, that the figure cannot be distinguished from a circle, he is included in the Circular or Priestly order; and this is the highest class of all.

It is a Law of Nature with us that a male child shall have one more side than his father, so that each generation shall rise (as a rule) one step in the scale of development and nobility. Thus the son of a Square is a Pentagon; the son of a Pentagon, a Hexagon; and so on.

But this rule applies not always to the Tradesman, and still less often to the Soldiers, and to the Workmen; who indeed can hardly be said to deserve the name of human Figures, since they have not all their sides equal. With them therefore the Law of Nature does not hold; and the son of an Isosceles (i.e. a Triangle with two sides equal) remains Isosceles still. Nevertheless, all hope is not such out, even from the Isosceles, that his posterity may ultimately rise above his degraded condition. For, after a long series of military successes, or diligent and skillful labours, it is generally found that the more intelligent among the Artisan and Soldier classes manifest a slight increase of their third side or base, and a shrinkage of the two other sides. Intermarriages (arranged by the Priests) between the sons and daughters of these more intellectual members of the lower classes generally result in an offspring approximating still more to the type of the Equal-Sided Triangle.

Rarely—in proportion to the vast numbers of Isosceles births—is a genuine and certifiable Equal-Sided Triangle produced from Isosceles parents.[1] Such a birth requires, as its antecedents, not only a series of carefully arranged intermarriages, but also a long-continued exercise of frugality and self-control on the part of the would-be ancestors of the coming Equilateral, and a patient, systematic, and continuous development of the Isosceles intellect through many generations.

[1] “What need of a certificate?” a Spaceland critic may ask: “Is not the procreation of a Square Son a certificate from Nature herself, proving the Equal-sidedness of the Father?” I reply that no Lady of any position will mary an uncertified Triangle. Square offspring has sometimes resulted from a slightly Irregular Triangle; but in almost every such case the Irregularity of the first generation is visited on the third; which either fails to attain the Pentagonal rank, or relapses to the Triangular.

The birth of a True Equilateral Triangle from Isosceles parents is the subject of rejoicing in our country for many furlongs round. After a strict examination conducted by the Sanitary and Social Board, the infant, if certified as Regular, is with solemn ceremonial admitted into the class of Equilaterals. He is then immediately taken from his proud yet sorrowing parents and adopted by some childless Equilateral, who is bound by oath never to permit the child henceforth to enter his former home or so much as to look upon his relations again, for fear lest the freshly developed organism may, by force of unconscious imitation, fall back again into his hereditary level.

The occasional emergence of an Equilateral from the ranks of his serf-born ancestors is welcomed, not only by the poor serfs themselves, as a gleam of light and hope shed upon the monotonous squalor of their existence, but also by the Aristocracy at large; for all the higher classes are well aware that these rare phenomena, while they do little or nothing to vulgarize their own privileges, serve as almost useful barrier against revolution from below.

Had the acute-angled rabble been all, without exception, absolutely destitute of hope and of ambition, they might have found leaders in some of their many seditious outbreaks, so able as to render their superior numbers and strength too much even for the wisdom of the Circles. But a wise ordinance of Nature has decreed that in proportion as the working-classes increase in intelligence, knowledge, and all virtue, in that same proportion their acute angle (which makes them physically terrible) shall increase also and approximate to their comparatively harmless angle of the Equilateral Triangle. Thus, in the most brutal and formidable off the soldier class—creatures almost on a level with women in their lack of intelligence—it is found that, as they wax in the mental ability necessary to employ their tremendous penetrating power to advantage, so do they wane in the power of penetration itself.

How admirable is the Law of Compensation! And how perfect a proof of the natural fitness and, I may almost say, the divine origin of the aristocratic constitution of the States of Flatland! By a judicious use of this Law of Nature, the Polygons and Circles are almost always able to stifle sedition in its very cradle, taking advantage of the irrepressible and boundless hopefulness of the human mind. Art also comes to the aid of Law and Order. It is generally found possible—by a little artificial compression or expansion on the part of the State physicians—to make some of the more intelligent leaders of a rebellion perfectly Regular, and to admit them at once into the privileged classes; a much larger number, who are still below the standard, allured by the prospect of being ultimately ennobled, are induced to enter the State Hospitals, where they are kept in honourable confinement for life; one or two alone of the most obstinate, foolish, and hopelessly irregular are led to execution.

Then the wretched rabble of the Isosceles, planless and leaderless, are either transfixed without resistance by the small body of their brethren whom the Chief Circle keeps in pay for emergencies of this kind; or else more often, by means of jealousies and suspicious skillfully fomented among them by the Circular party, they are stirred to mutual warfare, and perish by one another’s angles. No less than one hundred and twenty rebellions are recorded in our annals, besides minor outbreaks numbered at two hundred and thirty-five; and they have all ended thus.

-= The End =-";

fn main() -> glib::ExitCode {
    colog::init();

    let app = Application::builder()
        .application_id(APP_ID)
        .build();
    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {

    let font_manager = FontManager::new();

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("GTK Font Renderer")
        .build();

    let area = DrawingArea::default();
    area.set_hexpand(true);
    area.set_vexpand(true);
    area.set_draw_func(move |area, gtk_cr, width, _height| {
        // Red square to indicate stuff is being drawn on screen
        gtk_cr.set_source_rgba(1.0, 0.0, 0.0, 1.0);
        gtk_cr.rectangle(0.0, 0.0, 100.0, 100.0);
        let _ = gtk_cr.fill();

        // Here we set the font. We find the font, load it and convert it from freetype font face to a cairo font face.
        let info = font_manager.find(vec!["comic sans ms"], FontStyle::Normal).expect("Failed to find font");
        // let ft_face = font_manager.load(&info).expect("Failed to load font");

        // Layout works nicely with bounding boxes and alignment, but i can't seem to get the fontface to render
        let layout = create_layout(gtk_cr);
        let desc = create_font_description(&info, 14.0);
        layout.set_font_description(Some(&desc));

        layout.set_text(TEXT);
        layout.set_width(width * pango::SCALE);
        layout.set_alignment(pango::Alignment::Center);

        let cur_y = 200;
        let mut max_y = cur_y;

        // Create layout
        gtk_cr.set_source_rgba(1.0, 0.0, 1.0, 1.0);
        gtk_cr.move_to(0.0, cur_y as f64);
        show_layout(&gtk_cr, &layout);
        max_y += layout.pixel_size().1;

        // Nice bounding rectangle around the text
        gtk_cr.set_source_rgba(0.0, 0.0, 0.0, 1.0);
        gtk_cr.set_line_width(1.0);
        gtk_cr.rectangle(0.0, cur_y as f64, width as f64, max_y as f64 - cur_y as f64);
        let _ = gtk_cr.stroke();

        // Add a little bit of padding
        max_y += 25;
        let cur_y = max_y;

        // Display the next text in a different font
        let info = font_manager.find(vec!["arial"], FontStyle::Normal).expect("Failed to find font");
        let desc = create_font_description(&info, 12.0);

        layout.set_font_description(Some(&desc));
        gtk_cr.set_source_rgba(0.7, 0.2, 0.5, 1.0);
        gtk_cr.move_to(0.0, cur_y as f64);
        show_layout(&gtk_cr, &layout);
        max_y += layout.pixel_size().1;

        // Bounding box around the text again
        gtk_cr.set_source_rgba(0.0, 1.0, 1.0, 1.0);
        gtk_cr.set_line_width(3.0);
        gtk_cr.rectangle(0.0, cur_y as f64, width as f64, max_y as f64 - cur_y as f64);
        let _ = gtk_cr.stroke();

        // Get current position and add the layout height. This is the new height of the canvas in this drawing area so
        // we can scroll.
        area.set_content_height(max_y as i32 + 50);
    });

    // Of course, scrolling doesn't work... need to figure out why it doesn't work.
    let scroll = gtk4::ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Automatic)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .child(&area)
        .build();
    window.set_child(Some(&scroll));

    window.set_default_width(800);
    window.set_default_height(600);
    window.present();
}

/// Convert a fontInfo from freetype into a pango font description.
fn create_font_description(info: &FontInfo, size: f64) -> pango::FontDescription {
    let mut desc = pango::FontDescription::new();
    desc.set_family(&info.family.clone());

    desc.set_style(match info.style {
        FontStyle::Italic => pango::Style::Italic,
        FontStyle::Oblique => pango::Style::Oblique,
        FontStyle::Normal => pango::Style::Normal,
    });

    desc.set_size((size * pango::SCALE as f64) as i32);

    desc
}