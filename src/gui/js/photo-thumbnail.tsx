import * as React from 'react';
import { Photo } from './interfaces';
import { chooseIcon } from './photo';

export interface PhotoThumbnailProps {
  style: React.CSSProperties,
  isSelected: boolean,
  handleSelect: () => void,
  photo: Photo,
}

export function PhotoThumbnail(props: PhotoThumbnailProps) {
  return (
    <div
      style={props.style}
      className="photo"
      onClick={props.handleSelect}
      onKeyUp={props.handleSelect}
      role="option"
      aria-selected={props.isSelected}
      tabIndex={0}
    >
      <img src={props.photo.src} title={props.photo.path} alt="thumbnail" />
      {chooseIcon(props.photo)}
    </div>
  );
}
