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
      locationHistoryPath: props.locationHistoryPath,
      interpolate: props.interpolate,
      filterPhotos: false,
      currentPhoto: undefined,
      photos: props.photos
    };

    this.getAndStoreLocations = this.getAndStoreLocations.bind(this);
    this.handleFilterToggle = this.handleFilterToggle.bind(this);
    this.handleInterpolateToggle = this.handleInterpolateToggle.bind(this);
    this.handlePhotoSelect = this.handlePhotoSelect.bind(this);
    this.handleSuggestionApply = this.handleSuggestionApply.bind(this);
    this.handleSuggestionDiscard = this.handleSuggestionDiscard.bind(this);
    this.getNewRootPath = this.getNewRootPath.bind(this);
    this.getNewLocationHistory = this.getNewLocationHistory.bind(this);
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

  getNewRootPath() {
    return requests.getNewRootPath().then(responseBody => {
      const rootPath = responseBody.rootPath;
      this.setState(Object.assign({}, this.state, { rootPath }));

      return this.handleFilterToggle({
        target: {
          checked: this.state.filterPhotos
        }
      });
    });
  }

  getNewLocationHistory() {
    return requests.getNewLocationHistory().then(responseBody => {
      const locationHistoryPath = responseBody.locationHistoryPath;
      const photos = this.state.photos.map(photo =>
        Object.assign({}, photo, { loaded: false })
      );

      this.setState(
        Object.assign({}, this.state, {
          locationHistoryPath,
          photos
        })
      );

      this.sidebar.forceUpdate();
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

  handleInterpolateToggle(event) {
    const interpolate = event.target.checked;

    return requests.putInterpolate(interpolate).then(() => {
      this.setState(Object.assign({}, this.state, { interpolate }));
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
          <div id="paths">
            <div>Photos path: {this.state.rootPath}</div>
            <div>Location history path: {this.state.locationHistoryPath}</div>
          </div>
          <div>
            <div>
              <button onClick={this.getNewRootPath}>Select Root Path</button>
              <label htmlFor="suggestionsCheckbox">
                <input
                  type="checkbox"
                  id="suggestionsCheckbox"
                  checked={this.state.filterPhotos}
                  onChange={this.handleFilterToggle}
                />
                Show only photos with suggestions
              </label>
            </div>
            <div>
              <button onClick={this.getNewLocationHistory}>
                Select Location History
              </button>
              <label htmlFor="interpolateCheckbox">
                <input
                  type="checkbox"
                  id="interpolateCheckbox"
                  checked={this.state.interpolate}
                  onChange={this.handleInterpolateToggle}
                />
                Interpolate locations
              </label>
            </div>
          </div>
        </header>
        <div>
          <Sidebar
            ref={sidebar => {
              this.sidebar = sidebar;
            }}
            photos={this.state.photos}
            currentPhoto={this.state.currentPhoto}
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
  locationHistoryPath: PropTypes.string.isRequired,
  interpolate: PropTypes.bool.isRequired,
  photos: PropTypes.arrayOf(photoType).isRequired
};
