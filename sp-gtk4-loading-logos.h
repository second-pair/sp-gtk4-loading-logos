/*  *--<Preface>--*  //

 -=-  Author Details  -=-
 Blair Edwards
 My own work on my own dime.

 -=-  Part Of  -=-
 sp-gtk4-loading-logos

//  *--</Preface>--*  */

#ifndef SP_GTK4_LOADING_LOGOS_H
#define SP_GTK4_LOADING_LOGOS_H



//  *--<Definitions>--*  //

//  Includes
//#include <gtk/gtk.h>

//  Defines

//  Global Type Definitions

//  Global Structures

//  Truly Global Variables

//  Global Prototype Functions
GtkWidget* sp_gtk4_loading_logos_create_default ();
GtkWidget* sp_gtk4_loading_logos_create (int32_t anim_type);
void sp_gtk4_loading_logos_set_type (GtkWidget* logo, int32_t anim_type);
int32_t sp_gtk4_loading_logos_get_type (GtkWidget* logo);

//  *--</Definitions>--*  //



#endif
