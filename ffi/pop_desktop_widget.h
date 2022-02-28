#include <gtk/gtk.h>

typedef struct { } PopDesktopWidget;

PopDesktopWidget *pop_desktop_widget_new (GtkStack *stack);

void pop_desktop_widget_free (PopDesktopWidget *self);

/** Creates the GTK widget for this page */
GtkWidget *pop_desktop_widget_gis_dock_page (GtkWidget *header);

/** Localized title for this page */
char *pop_desktop_widget_gis_dock_title ();

/** Creates the GTK widget for this page */
GtkWidget *pop_desktop_widget_gis_extensions_page (GtkWidget *header);

/** Localized title for this page */
char *pop_desktop_widget_gis_extensions_title ();

/** Creates the GTK widget for this page */
GtkWidget *pop_desktop_widget_gis_gestures_page (GtkWidget *header);

/** Localized title for this page */
char *pop_desktop_widget_gis_gestures_title ();

/** Creates the GTK widget for this page */
GtkWidget *pop_desktop_widget_gis_launcher_page (GtkWidget *header);

/** Localized title for this page */
char *pop_desktop_widget_gis_launcher_title ();

/** Creates the GTK widget for this page */
GtkWidget *pop_desktop_widget_gis_panel_page (GtkWidget *header);

/** Localized title for this page */
char *pop_desktop_widget_gis_panel_title ();

/** Initializes this library's gresources */
void pop_desktop_widget_gresource_init ();

/** Initializes or reloads the localizer */
void pop_desktop_widget_localize ();

/** Frees strings created by this library */
void pop_desktop_widget_string_free (char *string);

GtkWidget* pop_desktop_widget_gcc_main_page (void);

GtkWidget* pop_desktop_widget_gcc_appearance_page (void);

GtkWidget* pop_desktop_widget_gcc_dock_page (void);

GtkWidget* pop_desktop_widget_gcc_workspaces_page (void);
