import * as React from 'react';
import { Photo } from './interfaces';
import { MainPanel } from './main-panel';
import * as requests from './requests';
import { PhotosGrid } from './photos-grid';

interface CheckboxEvent {
  target: {
    checked: boolean;
  };
}

export interface PageProps {
  interpolate: boolean;
  locationHistoryPath: string;
  photos: Photo[];
  rootPath: string;
}

export interface PageState {
  currentPhoto: Photo;
  filterPhotos: boolean;
  interpolate: boolean;
  locationHistoryPath: string;
  photos: Photo[];
  rootPath: string;
}

export class Page extends React.Component<PageProps, PageState> {
  private photosGrid: PhotosGrid;

  public constructor(props: PageProps) {
    super(props);

    this.state = {
      currentPhoto: undefined,
      filterPhotos: false,
      interpolate: props.interpolate,
      locationHistoryPath: props.locationHistoryPath,
      photos: props.photos,
      rootPath: props.rootPath
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

  public render() {
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
          <PhotosGrid
            ref={photosGrid => {
              this.photosGrid = photosGrid;
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

  private getLocationsPromise(startIndex: number, stopIndex: number) {
    if (this.state.filterPhotos) {
      const promises = [];
      for (let i = startIndex; i < stopIndex; i += 1) {
        promises.push(requests.getLocation(this.state.photos[i].path));
      }

      return Promise.all(promises);
    }

    return requests.getLocations(startIndex, stopIndex);
  }

  private getAndStoreLocations(startIndex: number, stopIndex: number) {
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

  private getNewRootPath() {
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

  private getNewLocationHistory() {
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

      this.photosGrid.forceUpdate();
    });
  }

  private handleFilterToggle(event: CheckboxEvent) {
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

  private handleInterpolateToggle(event: CheckboxEvent) {
    const interpolate = event.target.checked;

    return requests.putInterpolate(interpolate).then(() => {
      this.setState(Object.assign({}, this.state, { interpolate }));
    });
  }

  private handlePhotoSelect(photo: Photo) {
    this.setState(Object.assign({}, this.state, { currentPhoto: photo }));
    this.photosGrid.forceUpdate();
  }

  private handleSuggestionApply() {
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
        this.photosGrid.forceUpdate();
      });
  }

  private handleSuggestionDiscard() {
    const currentPhoto = Object.assign({}, this.state.currentPhoto);
    currentPhoto.location = undefined;

    const photos = this.state.photos.slice();
    const index = photos.findIndex(photo => photo.path === currentPhoto.path);
    photos[index] = currentPhoto;

    this.setState(Object.assign({}, this.state, { currentPhoto, photos }));
    this.photosGrid.forceUpdate();
  }
}
