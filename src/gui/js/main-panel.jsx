import React from 'react';
import PropTypes from 'prop-types';
import MapArea from './map-area';
import { locationDescription, hasSuggestion } from './photo';

export default function MainPanel(props) {
  if (props.photo) {
    return (
      <main>
        <section>
          <img src={props.photo.src} alt="Selected" />
          <MapArea photo={props.photo} />
        </section>
        <footer>
          {locationDescription(props.photo)}
          <br />
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
        </footer>
      </main>
    );
  }
  return (
    <main>
      <section>No photo selected.</section>
      <footer>
        <br />
        <button disabled>Apply</button>
        <button disabled>Discard</button>
      </footer>
    </main>
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