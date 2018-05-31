import * as React from 'react';
import { Photo } from './interfaces';
import { PhotoLocationViewer } from './photo-location-viewer';
import * as requests from './requests';
import { PhotosGrid } from './photos-grid';
import { updatePhotoLocations } from './photo';

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

function getLocationsPromise(photos: Photo[], arePhotosFiltered: boolean) {
  if (arePhotosFiltered) {
    const promises = [];
    for (let i = 0; i < photos.length; i += 1) {
      promises.push(requests.getLocation(photos[i].path));
    }

    return Promise.all(promises);
  }

  return requests.getLocations(0, photos.length);
}

function getAndStoreLocations(photos: Photo[], arePhotosFiltered: boolean) {
  return getLocationsPromise(photos, arePhotosFiltered).then(locations =>
    updatePhotoLocations(photos, locations)
  );
}

export class Page extends React.Component<PageProps, PageState> {
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
            <div>
              Photos path:
              {this.state.rootPath}
            </div>
            <div>
              Location history path:
              {this.state.locationHistoryPath}
            </div>
          </div>
          <div>
            <div>
              <button type="button" onClick={this.getNewRootPath}>
                Select Root Path
              </button>
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
              <button type="button" onClick={this.getNewLocationHistory}>
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
            photos={this.state.photos}
            currentPhoto={this.state.currentPhoto}
            handlePhotoSelect={this.handlePhotoSelect}
          />
          <PhotoLocationViewer
            photo={this.state.currentPhoto}
            handleSuggestionApply={this.handleSuggestionApply}
            handleSuggestionDiscard={this.handleSuggestionDiscard}
          />
        </div>
      </div>
    );
  }

  private getNewRootPath() {
    return requests.getNewRootPath().then(responseBody => {
      const rootPath = responseBody.rootPath;
      this.setState({ rootPath });

      return this.handleFilterToggle({
        target: {
          checked: this.state.filterPhotos
        }
      });
    });
  }

  private async getNewLocationHistory() {
    const { locationHistoryPath } = await requests.getNewLocationHistory();

    await getAndStoreLocations(this.state.photos, this.state.filterPhotos).then(
      photos => {
        this.setState({
          locationHistoryPath,
          photos
        });
      }
    );
  }

  private handleFilterToggle(event: CheckboxEvent) {
    const filterPhotos = event.target.checked;
    let promise;
    if (filterPhotos) {
      promise = requests.getFilteredPhotos();
    } else {
      promise = requests.getPhotos();
    }
    return promise
      .then(photos => getAndStoreLocations(photos, filterPhotos))
      .then(photos => {
        this.setState({ filterPhotos, photos });
      });
  }

  private handleInterpolateToggle(event: CheckboxEvent) {
    const interpolate = event.target.checked;

    return requests.putInterpolate(interpolate).then(() => {
      this.setState({ interpolate });
    });
  }

  private handlePhotoSelect(photo: Photo) {
    this.setState({ currentPhoto: photo });
  }

  private handleSuggestionApply() {
    return requests
      .writeCoordinates(
        this.state.currentPhoto.path,
        this.state.currentPhoto.location.Suggested[0]
      )
      .then(() => {
        this.setState(previousState => {
          const currentPhoto = Object.assign({}, previousState.currentPhoto, {
            location: {
              Existing: previousState.currentPhoto.location.Suggested[0]
            }
          });

          const photos = previousState.photos.slice();
          const index = photos.findIndex(
            photo => photo.path === currentPhoto.path
          );
          photos[index] = currentPhoto;

          return { currentPhoto, photos };
        });
      });
  }

  private handleSuggestionDiscard() {
    this.setState(previousState => {
      const currentPhoto = Object.assign({}, previousState.currentPhoto);
      currentPhoto.location = undefined;

      const photos = previousState.photos.slice();
      const index = photos.findIndex(photo => photo.path === currentPhoto.path);
      photos[index] = currentPhoto;

      return { currentPhoto, photos };
    });
  }
}
