function get(url) {
  return fetch(url).then(response => {
    if (response.ok) {
      return response.json();
    }
    throw new Error(`Failed to GET ${url}`);
  });
}

function put(url, body) {
  const init = {
    method: 'PUT',
    body: JSON.stringify(body)
  };

  return fetch(url, init).then(response => {
    if (!response.ok) {
      throw new Error(`Failed to ${init.method} ${url} with body ${init.body}`);
    }
  });
}

function setPhotoSrc(photo) {
  return Object.assign(photo, {
    src: `/thumbnail?path=${encodeURIComponent(
      photo.path
    )}&maxWidth=500&maxHeight=500`
  });
}

function mapPhotos(responseBody) {
  return responseBody.photos.map(setPhotoSrc);
}

export function getRootPath() {
  return get('/rootPath');
}

export function getPhotos() {
  return get('/photos').then(mapPhotos);
}

export function getFilteredPhotos() {
  return get('/photos?filter').then(mapPhotos);
}

export function writeCoordinates(path, coordinates) {
  return put(`/location?path=${encodeURIComponent(path)}`, coordinates);
}

export function getLocations(startIndex, endIndex) {
  return get(`/locations?start=${startIndex}&end=${endIndex}`).then(
    responseBody => responseBody.locations
  );
}

export function getLocation(path) {
  return get(`/location?path=${encodeURIComponent(path)}`);
}
