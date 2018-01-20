import React from 'react';
import PropTypes from 'prop-types';
import { chooseIcon } from './photo';

export default function PhotoThumbnail(props) {
  return (
    <div
      style={props.style}
      className="photo"
      onClick={props.handleSelect}
      onKeyUp={props.handleSelect}
      role="option"
      aria-selected={props.isSelected}
      tabIndex="0"
    >
      <img src={props.photo.src} title={props.photo.path} alt="thumbnail" />
      {chooseIcon(props.photo)}
    </div>
  );
}

PhotoThumbnail.propTypes = {
  style: PropTypes.object.isRequired, // eslint-disable-line react/forbid-prop-types
  isSelected: PropTypes.bool.isRequired,
  handleSelect: PropTypes.func.isRequired,
  photo: PropTypes.shape({
    path: PropTypes.string,
    src: PropTypes.string
  }).isRequired
};
