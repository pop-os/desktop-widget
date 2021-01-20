#include <gtk/gtk.h>

typedef struct { } PopDesktopWidget;

PopDesktopWidget *pop_desktop_widget_new (GtkStack *stack);

void pop_desktop_widget_free (PopDesktopWidget *self);
