#ifndef FLUTTER_NATIVE_WINDOW_STATE_H_
#define FLUTTER_NATIVE_WINDOW_STATE_H_

#include <cairo/cairo.h>
#include <gtk/gtk.h>

#include <vector>

#include "layer_shell_api.g.h"

struct RoundedRegionRect {
  cairo_rectangle_int_t rect = {0, 0, 0, 0};
  int radius_top_left = 0;
  int radius_top_right = 0;
  int radius_bottom_right = 0;
  int radius_bottom_left = 0;
};

struct HitRegionState {
  int bar_height = 0;
  HyprbaricNativeLayerShellBarEdge bar_edge =
      HYPRBARIC_NATIVE_LAYER_SHELL_BAR_EDGE_TOP;
  gboolean has_menu = FALSE;
  gboolean capture_all_clicks = FALSE;
  RoundedRegionRect menu;
  std::vector<RoundedRegionRect> regions;
};

struct NativeWindowState {
  GtkWindow *window = nullptr;
  GtkWidget *view = nullptr;
  gboolean layer_shell_available = FALSE;
  HitRegionState hit_region_state;
  HitRegionState last_applied_hit_region_state;
  gboolean hit_region_apply_scheduled = FALSE;
  int last_applied_region_width = -1;
  int last_applied_region_height = -1;
};

NativeWindowState *native_window_state_attach(GtkWindow *window,
                                              GtkWidget *view,
                                              gboolean layer_shell_available);

NativeWindowState *native_window_state_from_window(GtkWindow *window);

#endif // FLUTTER_NATIVE_WINDOW_STATE_H_
