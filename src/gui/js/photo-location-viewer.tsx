import * as React from 'react';
import { Photo } from './interfaces';
import { MapArea } from './map-area';
import { hasSuggestion, locationDescription } from './photo';

function getPhotoSrc(photo: Photo) {
  return `/thumbnail?path=${encodeURIComponent(
    photo.path
  )}&maxWidth=300&maxHeight=300`;
}

export interface PhotoLocationViewerProps {
  photo?: Photo;
  handleSuggestionApply: () => Promise<void>;
  handleSuggestionDiscard: () => void;
}

export function PhotoLocationViewer(props: PhotoLocationViewerProps) {
  if (props.photo) {
    return (
      <div id="main">
        <section>
          <img src={getPhotoSrc(props.photo)} alt="Selected" />
          <MapArea photo={props.photo} />
        </section>
        <footer>
          <div>{locationDescription(props.photo)}</div>
          <div>
            <button
              type="button"
              disabled={!hasSuggestion(props.photo)}
              onClick={props.handleSuggestionApply}
            >
              Apply
            </button>
            <button
              type="button"
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
          <button type="button" disabled>
            Apply
          </button>
          <button type="button" disabled>
            Discard
          </button>
        </div>
      </footer>
    </div>
  );
}
