
To Do
=====

Implement the base of the GObject heirarchy

Implement signal connection methods on GObject

  We will at least need one for each unique combination of return type and argument
  count. We should be able to avoid making a separate one for each GdkEvent type, since
  they can all share the same crust function.

Implement window and button widgets

Make a test app that shows a button in a window and handles the click

Figure out how to generate most of the API automatically

  GDK Events

  GTK Widgets

