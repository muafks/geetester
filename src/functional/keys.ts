import { mount, unmount } from "svelte";
import HelpWidget from "../widgetsComponents/helpWidget.svelte";
import OpenLinKDialog from "../widgetsComponents/openLinKDialog.svelte";
import LinkHistory from "../widgetsComponents/linkHistory.svelte";

export const commandKeyring = Symbol("commands");

export interface CommandStates {
  helpOpen: boolean;
}

export interface CommandsInKeyring {
  clas: Function; /// Clear all layers
  sc: Function; /// Shuffle the color
  ral: Function; // remove a layer by name
  scol: Function;
}

export class WidgetManager {
  private helpW: any = null;
  private helpOpen = false;

  propertiesExplorerOpen = false;
  propertiesExplorerData: any = {};

  private linkW: any = null;
  private linkOpen = false;

  private historyW: any = null;
  private historyOpen = false;

  onUpdate?: () => void;

  toggleHelp() {
    if (this.helpOpen) {
      console.log("close");

      if (this.helpW) {
        unmount(this.helpW, { outro: true });
        this.helpW = null;
      }

      this.helpOpen = false;
    } else {
      console.log("open");

      this.helpW = mount(HelpWidget, {
        target: document.body,
        props: {
          wM: this,
        },
      });

      this.helpOpen = true;
    }
  }

  toggleHistory() {
    if (this.historyOpen) {
      console.log("closeh");

      if (this.historyW) {
        unmount(this.historyW, { outro: true });
        this.historyW = null;
      }

      this.historyOpen = false;
    } else {
      console.log("openh");

      this.historyW = mount(LinkHistory, {
        target: document.body,
        props: {
          wM: this,
        },
      });

      this.historyOpen = true;
    }
  }

  clearProperties() {
    this.propertiesExplorerOpen = false;
    this.propertiesExplorerData = {};
    this.onUpdate?.();
  }

  toggleProperties(properties: any) {
    this.propertiesExplorerOpen = true;
    this.propertiesExplorerData = properties;
    this.onUpdate?.();
  }

  openLinkDialog(link: string | null) {
    if (!link) {
      this.linkOpen = false;
      unmount(this.linkW);
    }

    if (this.linkOpen) {
      unmount(this.linkW);
      this.linkOpen = false;
    }

    this.linkOpen = true;
    this.linkW = mount(OpenLinKDialog, {
      target: document.body,
      props: { link: link!, wM: this },
    });
    this.onUpdate?.();
  }
  closeLinkDialog() {
    if (!this.linkOpen) return;

    unmount(this.linkW);
    this.linkW = null;
    this.linkOpen = false;

    this.onUpdate?.();
  }
}

export interface UserQuestionPayload {
  question: string;
  options: string[];
  q_type: string;
}

export interface Source {
  s_id: string;
  s_type: string;
  data?: string | null;
  url?: string | null;
  tiles?: string[] | null;
  tile_size?: number | null;
  min_zoom?: number | null;
  max_zoom?: number | null;
}

export interface Layer {
  l_id: string;
  l_type: string;
  s_id: string;
  s_layer?: string | null;
}

export interface LayerSourceCollection {
  source: Source;
  layer: Layer;
}
