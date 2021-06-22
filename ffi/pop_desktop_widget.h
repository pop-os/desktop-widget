#include <gtk/gtk.h>

typedef struct { } PopDesktopWidget;

PopDesktopWidget *pop_desktop_widget_new (GtkStack *stack);

void pop_desktop_widget_free (PopDesktopWidget *self);

GtkWidget *pop_desktop_widget_gis_dock_page (GtkWidget *header);

GtkWidget *pop_desktop_widget_gis_panel_page (GtkWidget *header);

void pop_desktop_widget_gresource_init ();