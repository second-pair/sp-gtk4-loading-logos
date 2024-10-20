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

//  Defines
#define SP_GTK4_LOADING_LOGOS_ENUM_TYPE int8_t

//  Global Type Definitions

//  Global Structures

//  Truly Global Variables

//  Global Prototype Functions
GtkWidget* sp_gtk4_loading_logos_create_default ();
GtkWidget* sp_gtk4_loading_logos_create (int32_t anim_type);
void sp_gtk4_loading_logos_set_type (GtkWidget* logo, SP_GTK4_LOADING_LOGOS_ENUM_TYPE anim_type);
SP_GTK4_LOADING_LOGOS_ENUM_TYPE sp_gtk4_loading_logos_get_type (GtkWidget* logo);
SP_GTK4_LOADING_LOGOS_ENUM_TYPE sp_gtk4_loading_logos_max_type (void);

//  *--</Definitions>--*  //



#endif
