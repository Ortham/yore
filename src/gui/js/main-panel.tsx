import * as React from 'react';
import { Photo } from './interfaces';
import { MapArea } from './map-area';
import { hasSuggestion, locationDescription } from './photo';

export interface MainPanelProps {
  photo?: Photo;
  handleSuggestionApply: () => Promise<void>;
  handleSuggestionDiscard: () => void;
}

export function MainPanel(props: MainPanelProps) {
  if (props.photo) {
    return (
      <div id="main">
        <section>
          <img src={props.photo.src} alt="Selected" />
          <MapArea photo={props.photo} />
        </section>
        <footer>
          <div>{locationDescription(props.photo)}</div>
          <div>
            <button
              disabled={!hasSuggestion(props.photo)}
              onClick={props.handleSuggestionApply}
            >
              Apply
            </button>
            <button
              disabled={!hasSuggestion(props.photo)}
              onClick={props.handleSuggestionDiscard}
            >
              Discard
            </button>
          </div>
        </footer>
      </div>
    );
  }
  return (
    <div id="main">
      <section>No photo selected.</section>
      <footer>
        <div />
        <div>
          <button disabled>Apply</button>
          <button disabled>Discard</button>
        </div>
      </footer>
    </div>
  );
}
