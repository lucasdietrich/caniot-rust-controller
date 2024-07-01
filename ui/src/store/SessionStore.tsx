import EventEmitter from "events";

const UIDarkModeKey = "UIDarkMode";
const UIdebugModeKey = "UIdebugMode";

// Store local data in the user's session (localStorage)
// These settings are specific to the user
class SessionStore extends EventEmitter {
  constructor() {
    super();
  }

  getUIDarkMode = (): boolean | null => {
    return localStorage.getItem(UIDarkModeKey) === "true";
  };

  setUIDarkMode = (darkMode: boolean) => {
    localStorage.setItem(UIDarkModeKey, darkMode.toString());
  };

  getUIDebugMode = (): boolean | null => {
    return localStorage.getItem(UIdebugModeKey) === "true";
  };

  setUIDebugMode = (debugMode: boolean) => {
    localStorage.setItem(UIdebugModeKey, debugMode.toString());
  };
}

const sessionStore = new SessionStore();
export default sessionStore;
