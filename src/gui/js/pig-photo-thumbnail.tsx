import * as React from 'react';
import * as ReactDOM from 'react-dom';
import { PhotoThumbnail } from './photo-thumbnail';
import { Photo } from './interfaces';

interface PigImage {
  aspectRatio: number;
  style: PigImageStyle;

  load(): void;
  hide(): void;
}

interface PigImageStyle {
  width: number;
  height: number;
  translateX: number;
  translateY: number;
  transition: string;
}

// This class is adapted from the ProgressiveImage prototype used in pig.js.
export default class PigPhotoThumbnail implements PigImage {
  public aspectRatio: number;

  public filename: string;

  public style: PigImageStyle;

  private existsOnPage: boolean;

  private pig: Pig;

  private classNames: PigImageClassNames;

  private element: HTMLElement;

  private isSelected: boolean;

  private photo: Photo;

  private handlePhotoSelect: (photo: Photo) => void;

  public constructor(singleImageData: PigImageData, _index: number, pig: Pig) {
    // Global State
    this.existsOnPage = false; // True if the element exists on the page.

    // Instance information
    this.aspectRatio = singleImageData.aspectRatio; // Aspect Ratio
    this.filename = singleImageData.filename; // Filename

    // The Pig instance
    this.pig = pig;

    this.classNames = {
      figure: `${pig.settings.classPrefix}-figure`,
      loaded: `${pig.settings.classPrefix}-loaded`
    };

    this.photo = singleImageData.photo;
    this.isSelected = singleImageData.isSelected;
    this.handlePhotoSelect = singleImageData.handlePhotoSelect;

    return this;
  }

  public load() {
    // Create a new image element, and insert it into the DOM. It doesn't
    // matter the order of the figure elements, because all positioning
    // is done using transforms.
    this.existsOnPage = true;
    this.updateStyles();
    this.pig.container.appendChild(this.getElement());

    // We run the rest of the function in a 100ms setTimeout so that if the
    // user is scrolling down the page very fast and hide() is called within
    // 100ms of load(), the hide() function will set this.existsOnPage to false
    // and we can exit.
    setTimeout(() => {
      // Guard against hide() being called before the timeout expired.
      if (this.existsOnPage) {
        this.render();
      }
    }, 100);
  }

  public hide() {
    // Remove the images from the element, so that if a user is scrolling super
    // fast, we won't try to load every image we scroll past.
    if (this.getElement()) {
      ReactDOM.unmountComponentAtNode(this.getElement());
    }

    // Remove the image from the DOM.
    if (this.existsOnPage) {
      this.pig.container.removeChild(this.getElement());
    }

    this.existsOnPage = false;
  }

  public update(singleImageData: PigImageData) {
    if (
      this.filename !== singleImageData.filename ||
      this.aspectRatio !== singleImageData.aspectRatio
    ) {
      this.hide();

      this.aspectRatio = singleImageData.aspectRatio;
      this.filename = singleImageData.filename;
    }

    if (
      this.photo !== singleImageData.photo ||
      this.isSelected !== singleImageData.isSelected ||
      this.handlePhotoSelect !== singleImageData.handlePhotoSelect
    ) {
      this.photo = singleImageData.photo;
      this.isSelected = singleImageData.isSelected;
      this.handlePhotoSelect = singleImageData.handlePhotoSelect;
      this.render();
    }
  }

  private getElement() {
    if (!this.element) {
      this.element = document.createElement(this.pig.settings.figureTagName);
      this.element.className = this.classNames.figure;
      this.updateStyles();
    }

    return this.element;
  }

  private updateStyles() {
    this.getElement().style.transition = 'none';
    this.getElement().style.width = `${this.style.width}px`;
    this.getElement().style.height = `${this.style.height}px`;
    this.getElement().style.transform = `translate3d(${
      this.style.translateX
    }px,${this.style.translateY}px, 0)`;
  }

  private render() {
    const src = this.pig.settings.urlForSize(
      this.filename,
      this.pig.settings.getImageSize(this.pig.lastWindowWidth)
    );

    const photoThumbnail = (
      <PhotoThumbnail
        key={this.filename}
        isSelected={this.isSelected}
        photo={this.photo}
        src={src}
        handleSelect={() => this.handlePhotoSelect(this.photo)}
        loadedClassName={this.classNames.loaded}
      />
    );

    ReactDOM.render(photoThumbnail, this.getElement());
  }
}

interface PigImageClassNames {
  figure: string;
  loaded: string;
}

interface PigImageData {
  filename: string;
  aspectRatio: number;
  photo: Photo;
  handlePhotoSelect: (photo: Photo) => void;
  isSelected: boolean;
}

interface Pig {
  settings: PigSettings;
  container: HTMLElement;
  lastWindowWidth: number;
}

interface PigSettings {
  classPrefix: string;
  figureTagName: string;
  urlForSize: (filename: string, thumbnailSize: number) => string;
  getImageSize: (lastWindowWidth: number) => number;
}
