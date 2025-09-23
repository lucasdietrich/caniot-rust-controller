import EventEmitter from "events";
import { cp } from "fs";

export interface ShortcutConfig {
  name: string;
  url: string;
  menu_order?: "after_home" | "before_settings" | "end";
  order?: number;
  icon?: string; // symbolic icon name, frontend maps to actual component
  target_blank?: boolean;
  // Badge configuration (explicit). If omitted, legacy defaults may apply (see Menu.tsx)
  badge?: boolean; // show a badge (dot style) around the icon
  badge_color?: string; // antd preset color token or hex (#RRGGBB)
}

export interface UiConfigResponse {
  shortcut?: ShortcutConfig[];
  message?: string;
}

export type UiConfigState = {
  loading: boolean;
  error: string | null;
  data: UiConfigResponse | null;
  lastUpdated: number | null;
};

const DEFAULT_STATE: UiConfigState = {
  loading: false,
  error: null,
  data: null,
  lastUpdated: null,
};

const UI_CONFIG_EVENT = "UI_CONFIG_EVENT";

class UIConfigStore extends EventEmitter {
  private state: UiConfigState = { ...DEFAULT_STATE };
  private inFlight: AbortController | null = null;

  getState(): UiConfigState {
    return this.state;
  }

  private setState(patch: Partial<UiConfigState>, emit: boolean = true) {
    this.state = { ...this.state, ...patch };
    if (emit) this.emit(UI_CONFIG_EVENT);
  }

  subscribe(callback: () => void) {
    this.on(UI_CONFIG_EVENT, callback);
    return () => this.removeListener(UI_CONFIG_EVENT, callback);
  }

  async fetch(force = false) {
    if (this.state.loading) return;
    if (!force && this.state.data) return; // already loaded
    this.setState({ loading: true, error: null });

    if (this.inFlight) {
      this.inFlight.abort();
    }
    this.inFlight = new AbortController();

    try {
      const resp = await fetch("/api/uiconfig", { signal: this.inFlight.signal });
      if (!resp.ok) {
        throw new Error(`Failed to fetch UI config: ${resp.status} ${resp.statusText}`);
      }
      const json = (await resp.json()) as UiConfigResponse;
      console.log("Fetched UI config:", json);
      this.setState({ data: json, loading: false, lastUpdated: Date.now() });
    } catch (e: any) {
      if (e.name === "AbortError") {
        return; // ignore aborted
      }
      this.setState({ error: e.message || "Unknown error", loading: false });
    }
  }
}

const uiConfigStore = new UIConfigStore();
export default uiConfigStore;
