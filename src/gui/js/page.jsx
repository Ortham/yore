import React from 'react';
import PropTypes from 'prop-types';
import MainPanel from './main-panel';
import Sidebar from './sidebar';
import * as requests from './requests';

export default class Page extends React.Component {
  constructor(props) {
    super(props);

    this.state = {
      rootPath: props.rootPath,
      filterPhotos: false,
      currentPhoto: undefined,
      photos: props.photos
    };

    this.handleFilterToggle = this.handleFilterToggle.bind(this);
    this.handlePhotoSelect = this.handlePhotoSelect.bind(this);
    this.getAndStoreLocations = this.getAndStoreLocations.bind(this);
    this.handleSuggestionApply = this.handleSuggestionApply.bind(this);
    this.handleSuggestionDiscard = this.handleSuggestionDiscard.bind(this);
  }

  getLocationsPromise(startIndex, stopIndex) {
    if (this.state.filterPhotos) {
      const promises = [];
      for (let i = startIndex; i < stopIndex; i += 1) {
        promises.push(requests.getLocation(this.state.photos[i].path));
      }

      return Promise.all(promises);
    }

    return requests.getLocations(startIndex, stopIndex);
  }

  getAndStoreLocations(startIndex, stopIndex) {
    return this.getLocationsPromise(startIndex, stopIndex).then(locations => {
      const photos = this.state.photos.slice();

      for (let i = startIndex; i < stopIndex; i += 1) {
        // Don't mutate the existing object.
        photos[i] = Object.assign({}, photos[i]);

        // Assign these here instead of using Object.assign to set any undefined
        // values.
        photos[i].location = locations[i - startIndex].location;
        photos[i].error = locations[i - startIndex].error;
        photos[i].loaded = true;
      }

      this.setState(Object.assign({}, this.state, { photos }));
    });
  }

  handleFilterToggle(event) {
    const filterPhotos = event.target.checked;
    let promise;
    if (filterPhotos) {
      promise = requests.getFilteredPhotos();
    } else {
      promise = requests.getPhotos();
    }
    return promise.then(photos => {
      this.setState(Object.assign({}, this.state, { filterPhotos, photos }));
    });
  }

  handlePhotoSelect(photo) {
    this.setState(Object.assign({}, this.state, { currentPhoto: photo }));
    this.sidebar.forceUpdate();
  }

  handleSuggestionApply() {
    return requests
      .writeCoordinates(
        this.state.currentPhoto.path,
        this.state.currentPhoto.location.Suggested[0]
      )
      .then(() => {
        const currentPhoto = Object.assign({}, this.state.currentPhoto, {
          location: {
            Existing: this.state.currentPhoto.location.Suggested[0]
          }
        });

        const photos = this.state.photos.slice();
        const index = photos.findIndex(
          photo => photo.path === currentPhoto.path
        );
        photos[index] = currentPhoto;

        this.setState(Object.assign({}, this.state, { currentPhoto, photos }));
        this.sidebar.forceUpdate();
      });
  }

  handleSuggestionDiscard() {
    const currentPhoto = Object.assign({}, this.state.currentPhoto);
    currentPhoto.location = undefined;

    const photos = this.state.photos.slice();
    const index = photos.findIndex(photo => photo.path === currentPhoto.path);
    photos[index] = currentPhoto;

    this.setState(Object.assign({}, this.state, { currentPhoto, photos }));
    this.sidebar.forceUpdate();
  }

  render() {
    return (
      <div>
        <header id="titleBar">
          <h1>Yore</h1>
          <div>{this.state.rootPath}</div>
        </header>
        <div>
          <Sidebar
            ref={sidebar => {
              this.sidebar = sidebar;
            }}
            photos={this.state.photos}
            filterPhotos={this.state.filterPhotos}
            currentPhoto={this.state.currentPhoto}
            handleFilterToggle={this.handleFilterToggle}
            handlePhotoSelect={this.handlePhotoSelect}
            getAndStoreLocations={this.getAndStoreLocations}
          />
          <MainPanel
            photo={this.state.currentPhoto}
            handleSuggestionApply={this.handleSuggestionApply}
            handleSuggestionDiscard={this.handleSuggestionDiscard}
          />
        </div>
      </div>
    );
  }
}

const photoType = PropTypes.shape({
  height: PropTypes.number,
  width: PropTypes.number,
  loaded: PropTypes.bool,
  path: PropTypes.string,
  src: PropTypes.string
});

Page.propTypes = {
  rootPath: PropTypes.string.isRequired,
  photos: PropTypes.arrayOf(photoType).isRequired
};
