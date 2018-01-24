import 'babel-polyfill';
import 'whatwg-fetch';
import React from 'react';
import ReactDOM from 'react-dom';
import Page from './page';
import {
  getInterpolate,
  getLocationHistoryPath,
  getPhotos,
  getRootPath
} from './requests';

function getInitialState() {
  const state = {
    rootPath: undefined,
    filterPhotos: false,
    currentPhoto: undefined,
    photos: []
  };

  return getRootPath()
    .then(responseBody => {
      state.rootPath = responseBody.rootPath || '';

      return getPhotos();
    })
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
  ReactDOM.render(
    <Page
      rootPath={state.rootPath}
      locationHistoryPath={state.locationHistoryPath}
      photos={state.photos}
      interpolate={state.interpolate}
    />,
    document.getElementById('root')
  );
});
