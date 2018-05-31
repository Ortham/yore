import '@babel/polyfill';
import * as React from 'react';
import * as ReactDOM from 'react-dom';
import 'whatwg-fetch';
import { Page, PageProps } from './page';
import {
  getInterpolate,
  getLocationHistoryPath,
  getPhotos,
  getRootPath,
  getLocations
} from './requests';
import { updatePhotoLocations } from './photo';

function getInitialState() {
  const state: PageProps = {
    interpolate: false,
    locationHistoryPath: undefined,
    photos: [],
    rootPath: undefined
  };

  return getRootPath()
    .then(responseBody => {
      state.rootPath = responseBody.rootPath || '';

      return getPhotos();
    })
    .then(photos =>
      getLocations(0, photos.length).then(locations =>
        updatePhotoLocations(photos, locations)
      )
    )
    .then(photos => {
      state.photos = photos;

      return getInterpolate();
    })
    .then(responseBody => {
      state.interpolate = responseBody.interpolate;

      return getLocationHistoryPath();
    })
    .then(responseBody => {
      state.locationHistoryPath = responseBody.locationHistoryPath || '';

      return state;
    });
}

getInitialState().then(state => {
  ReactDOM.render(<Page {...state} />, document.getElementById('root'));
});
