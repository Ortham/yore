import React from 'react';
import PropTypes from 'prop-types';
import MainPanel from './main-panel';
import Sidebar from './sidebar';

export default class Page extends React.Component {
  constructor(props) {
    super(props);

    this.state = {
      rootPath: props.rootPath || undefined,
      filterPhotos: false,
      currentPhoto: undefined,
      photos: props.photos,
      sidebarPhotos: props.photos
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
        const promise = this.props.getLocation(
          this.state.sidebarPhotos[i].path
        );
        promises.push(promise);
      }

      return Promise.all(promises);
    }

    return this.props.getLocations(startIndex, stopIndex);
  }

  getAndStoreLocations(startIndex, stopIndex) {
    return this.getLocationsPromise(startIndex, stopIndex).then(locations => {
      const sidebarPhotos = this.state.sidebarPhotos.slice();

      for (let i = startIndex; i < stopIndex; i += 1) {
        sidebarPhotos[i].location = locations[i - startIndex].location;
        sidebarPhotos[i].error = locations[i - startIndex].error;
        sidebarPhotos[i].loaded = true;
      }

      this.setState(Object.assign({}, this.state, { sidebarPhotos }));
    });
  }

  handleFilterToggle(event) {
    const checked = event.target.checked;
    let promise;
    if (checked) {
      promise = this.props.getFilteredPhotos(this.state.photos);
    } else {
      promise = Promise.resolve(this.state.photos);
    }
    return promise.then(photos => {
      this.setState(
        Object.assign({}, this.state, {
          filterPhotos: checked,
          sidebarPhotos: photos
        })
      );
    });
  }

  handlePhotoSelect(photo) {
    this.setState(Object.assign({}, this.state, { currentPhoto: photo }));
    this.sidebar.forceUpdate();
  }

  handleSuggestionApply() {
    return this.props
      .writeCoordinates(
        this.state.currentPhoto.path,
        this.state.currentPhoto.location.Suggested[0]
      )
      .then(() => {
        const sidebarPhotos = this.state.sidebarPhotos.slice();
        const currentPhoto = sidebarPhotos.find(
          photo => photo.path === this.state.currentPhoto.path
        );
        currentPhoto.location.Existing = currentPhoto.location.Suggested[0];
        currentPhoto.location.Suggested = undefined;

        this.setState(
          Object.assign({}, this.state, { currentPhoto, sidebarPhotos })
        );
        this.sidebar.forceUpdate();
      });
  }

  handleSuggestionDiscard() {
    const sidebarPhotos = this.state.sidebarPhotos.slice();
    const currentPhoto = sidebarPhotos.find(
      photo => photo.path === this.state.currentPhoto.path
    );
    currentPhoto.location = undefined;

    this.setState(
      Object.assign({}, this.state, { currentPhoto, sidebarPhotos })
    );
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
            photos={this.state.sidebarPhotos}
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
  photos: PropTypes.arrayOf(photoType).isRequired,
  writeCoordinates: PropTypes.func.isRequired,
  getLocations: PropTypes.func.isRequired,
  getLocation: PropTypes.func.isRequired,
  getFilteredPhotos: PropTypes.func.isRequired
};
