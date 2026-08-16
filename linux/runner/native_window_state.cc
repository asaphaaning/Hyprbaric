#include "native_window_state.h"

namespace {
constexpr char kNativeWindowStateKey[] = "hyprbaric-native-window-state";

void native_window_state_free(gpointer data) {
  delete static_cast<NativeWindowState *>(data);
}
} // namespace

NativeWindowState *native_window_state_attach(GtkWindow *window,
                                              GtkWidget *view,
                                              gboolean layer_shell_available) {
  auto *state = new NativeWindowState();
  state->window = window;
  state->view = view;
  state->layer_shell_available = layer_shell_available;

  g_object_set_data_full(G_OBJECT(window), kNativeWindowStateKey, state,
                         native_window_state_free);
  return state;
}

NativeWindowState *native_window_state_from_window(GtkWindow *window) {
  if (window == nullptr) {
    return nullptr;
  }
  return static_cast<NativeWindowState *>(
      g_object_get_data(G_OBJECT(window), kNativeWindowStateKey));
}
