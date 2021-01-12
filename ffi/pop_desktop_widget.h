#include <gtk/gtk.h>

typedef struct { } PopDesktopWidget;

PopDesktopWidget *pop_desktop_widget_new (void);

void pop_desktop_widget_grab_focus (const PopDesktopWidget *self);

GtkWidget *pop_desktop_widget_widget (const PopDesktopWidget *self);

void pop_desktop_widget_free (PopDesktopWidget *self);
