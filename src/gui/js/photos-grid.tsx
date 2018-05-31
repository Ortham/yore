import * as React from 'react';
import Pig from 'pig.js/src/pig.js';
import { Photo } from './interfaces';
import PigPhotoThumbnail from './pig-photo-thumbnail';

export interface PhotosGridProps {
  photos: Photo[];
  currentPhoto?: Photo;
  handlePhotoSelect: (photo: Photo) => void;
}

export class PhotosGrid extends React.Component<PhotosGridProps, {}> {
  private pig: any;

  public componentDidMount() {
    const imageData = this.getPigImageData();

    const photosList = document.getElementById('photosList');

    const options = {
      containerId: 'pig',
      scroller: photosList,
      imageType: PigPhotoThumbnail,

      // Width is 10*height to avoid getting a thumbnail constrained by its
      // width.
      urlForSize: (filename: string, height: number) =>
        `/thumbnail?path=${encodeURIComponent(filename)}&maxWidth=${10 *
          height}&maxHeight=${height}`
    };

    this.pig = new Pig(imageData, options).enable();
    // Immediately call update adjust layout to account for scroll bar.
    this.pig.update(imageData);
  }

  public componentWillUnmount() {
    this.pig.disable();
  }

  public componentDidUpdate(/* prevProps, prevState, snapshot */) {
    if (this.pig) {
      this.pig.update(this.getPigImageData());
    }
  }

  public render() {
    return (
      <nav id="photosList">
        <div id="pig" style={{ width: '100%' }} />
      </nav>
    );
  }

  private getPigImageData() {
    return this.props.photos.map(photo => ({
      filename: photo.path,
      aspectRatio: photo.width / photo.height,
      photo,
      isSelected: photo === this.props.currentPhoto,
      handlePhotoSelect: this.props.handlePhotoSelect
    }));
  }
}
