Automatically Generated Bindings
================================

* A three-phase process:
  1. C header files are converted to an intermediate format
  2. A manually-authored overlay is applied to the intermediate document
  3. The resulting intermediate document is converted to Rust bindings

* Use JSON for the intermediate format
* Top-level items will be:
  * Classes in the GObject heirarchy
  * GDK event types
  * Enumeration types
  * Other types: the manual overlay will provide mappings for these types
