import * as React from 'react';
import { Photo } from './interfaces';
import { MapArea } from './map-area';
import { hasSuggestion, locationDescription } from './photo';

function getPhotoSrc(photo: Photo) {
  // Could get width from element width, but use a large value to allow for
  // resizing - 900 is > the width of the container in a fullscreen window on
  // a 2560x1440 screen.
  const maxWidth = 900;
  const maxHeight = Math.ceil((photo.height / photo.width) * maxWidth);
  return `/thumbnail?path=${encodeURIComponent(
    photo.path
  )}&maxWidth=${maxWidth}&maxHeight=${maxHeight}`;
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
