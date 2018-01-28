import React from 'react';
import PropTypes from 'prop-types';
import MapArea from './map-area';
import { locationDescription, hasSuggestion } from './photo';

export default function MainPanel(props) {
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

MainPanel.propTypes = {
  photo: PropTypes.shape({
    src: PropTypes.string
  }),
  handleSuggestionApply: PropTypes.func.isRequired,
  handleSuggestionDiscard: PropTypes.func.isRequired
};

MainPanel.defaultProps = {
  photo: undefined
};
