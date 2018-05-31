import * as React from 'react';
import { Photo } from './interfaces';
import { chooseIcon } from './photo';

export interface PhotoThumbnailProps {
  isSelected: boolean;
  photo: Photo;
  src: string;
  loadedClassName: string;
  handleSelect: () => void;
}

export function PhotoThumbnail(props: PhotoThumbnailProps) {
  return (
    <div
      className="photo"
      onClick={props.handleSelect}
      onKeyUp={props.handleSelect}
      role="option"
      aria-selected={props.isSelected}
      tabIndex={0}
    >
      <img
        src={props.src}
        title={props.photo.path}
        alt="thumbnail"
        onLoad={evt => {
          if (evt.target instanceof HTMLElement) {
            evt.target.parentElement.classList.add(props.loadedClassName);
          }
        }}
      />
      {chooseIcon(props.photo)}
    </div>
  );
}
