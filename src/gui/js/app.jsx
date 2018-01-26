import 'babel-polyfill';
import 'whatwg-fetch';
import React from 'react';
import ReactDOM from 'react-dom';
import Page from './page';
import { getPhotos, getRootPath } from './requests';

function getInitialState() {
  const state = {
    rootPath: undefined,
    filterPhotos: false,
    currentPhoto: undefined,
    photos: []
  };

  return getRootPath()
    .then(responseBody => {
      state.rootPath = responseBody.rootPath;

      return getPhotos();
    })
    .then(photos => {
      state.photos = photos;

      return state;
    });
}

getInitialState().then(state => {
  ReactDOM.render(
    <Page rootPath={state.rootPath} photos={state.photos} />,
    document.getElementById('root')
  );
});
