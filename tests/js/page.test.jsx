import React from 'react';
import renderer from 'react-test-renderer';

jest.mock('../../src/gui/js/main-panel', () => 'MainPanel');
jest.mock('../../src/gui/js/sidebar', () => 'Sidebar');
jest.unmock('../../src/gui/js/requests');

import * as requests from '../../src/gui/js/requests'; // eslint-disable-line import/first
import Page from '../../src/gui/js/page'; // eslint-disable-line import/first

describe('Page', () => {
  const photos = [
    {
      height: 5,
      width: 10,
      loaded: false,
      path: 'path 1',
      src: 'source 1'
    },
    {
      height: 5,
      width: 10,
      loaded: true,
      path: 'path 2',
      src: 'source 2',
      location: {
        Suggested: [
          {
            latitude: 52.0,
            longitude: 13.2
          },
          {
            meters: 5,
            seconds: 20
          }
        ]
      }
    }
  ];
  let page;

  beforeAll(() => {
    requests.writeCoordinates = jest
      .fn()
      .mockReturnValueOnce(Promise.resolve());
    requests.getFilteredPhotos = jest
      .fn()
      .mockReturnValueOnce(Promise.resolve([photos[1]]));
    requests.getLocation = jest.fn().mockReturnValue(Promise.resolve());
    requests.getLocations = jest.fn().mockReturnValue(
      Promise.resolve([
        { error: 'Oh no!' },
        {
          location: {
            Existing: {
              latitude: 5,
              longitude: 30
            }
          }
        }
      ])
    );
    requests.getPhotos = jest.fn().mockReturnValue(Promise.resolve(photos));

    page = renderer.create(<Page rootPath="" photos={photos} />, {
      createNodeMock: element => {
        if (element.type === 'Sidebar') {
          return {
            forceUpdate: jest.fn()
          };
        }
        return null;
      }
    });
  });

  beforeEach(() => {
    requests.writeCoordinates.mockClear();
    requests.getFilteredPhotos.mockClear();
    requests.getLocation.mockClear();
    requests.getLocations.mockClear();
    requests.getPhotos.mockClear();

    page.root.instance.state.photos = photos;
    page.root.instance.state.currentPhoto = undefined;
  });

  test('renders a header, sidebar and main panel', () => {
    expect(page.toJSON()).toMatchSnapshot();
  });

  test('handlePhotoSelect should set the current photo to the given photo', () => {
    const pageInstance = page.root.instance;

    expect(pageInstance.state.currentPhoto).toBe(undefined);

    pageInstance.handlePhotoSelect(pageInstance.state.photos[1]);

    expect(pageInstance.state.currentPhoto).toBe(pageInstance.state.photos[1]);
    expect(pageInstance.sidebar.forceUpdate.mock.calls.length).toBe(1);
  });

  test('handleSuggestionApply calls writeCoordinates then moves suggested location to existing', () => {
    const pageInstance = page.root.instance;

    pageInstance.handlePhotoSelect(pageInstance.state.photos[1]);

    const currentPhoto = pageInstance.state.currentPhoto;
    const path = currentPhoto.path;
    const coordinates = currentPhoto.location.Suggested[0];

    return pageInstance.handleSuggestionApply().then(() => {
      expect(requests.writeCoordinates.mock.calls.length).toBe(1);
      expect(requests.writeCoordinates.mock.calls[0].length).toBe(2);
      expect(requests.writeCoordinates.mock.calls[0][0]).toBe(path);
      expect(requests.writeCoordinates.mock.calls[0][1]).toBe(coordinates);
      expect(pageInstance.sidebar.forceUpdate.mock.calls.length).toBe(1);

      expect(pageInstance.state.currentPhoto).not.toBe(currentPhoto);
      expect(pageInstance.state.photos).not.toBe(photos);

      expect(pageInstance.state.currentPhoto.location.Suggested).toBe(
        undefined
      );
      expect(pageInstance.state.currentPhoto.location.Existing).toBe(
        coordinates
      );

      expect(pageInstance.state.photos.length).toBe(2);
      expect(pageInstance.state.photos[0]).toBe(photos[0]);
      expect(pageInstance.state.photos[1]).toBe(
        pageInstance.state.currentPhoto
      );
    });
  });

  test("handleSuggestionDiscard should set the current photo's location to be undefined", () => {
    const pageInstance = page.root.instance;

    pageInstance.handlePhotoSelect(pageInstance.state.photos[1]);

    pageInstance.handleSuggestionDiscard();

    expect(pageInstance.state.currentPhoto).not.toBe(photos[1]);
    expect(pageInstance.state.currentPhoto.location).toBe(undefined);

    expect(pageInstance.state.photos).not.toBe(photos);
    expect(pageInstance.state.photos[0]).toBe(photos[0]);
    expect(pageInstance.state.photos[1]).toBe(pageInstance.state.currentPhoto);

    expect(pageInstance.sidebar.forceUpdate.mock.calls.length).toBe(1);
  });

  test('handleFilterToggle should call getFilteredPhotos if the filter is enabled', () => {
    const pageInstance = page.root.instance;

    expect(pageInstance.state.filterPhotos).toBe(false);

    return pageInstance
      .handleFilterToggle({ target: { checked: true } })
      .then(() => {
        expect(requests.getFilteredPhotos.mock.calls.length).toBe(1);

        expect(pageInstance.state.filterPhotos).toBe(true);
        expect(pageInstance.state.photos.length).toBe(1);
        expect(pageInstance.state.photos[0]).toEqual(photos[1]);
      });
  });

  test('handleFilterToggle should call set full photos array if the filter is disabled', () => {
    const pageInstance = page.root.instance;

    return pageInstance
      .handleFilterToggle({ target: { checked: false } })
      .then(() => {
        expect(requests.getFilteredPhotos.mock.calls.length).toBe(0);
        expect(pageInstance.state.filterPhotos).toBe(false);
        expect(pageInstance.state.photos).toEqual(photos);
      });
  });

  test('getLocationsPromise calls getLocation for each photo if filterPhotos is true', () => {
    const pageInstance = page.root.instance;

    pageInstance.setState(
      Object.assign({}, pageInstance.state, { filterPhotos: true })
    );

    return pageInstance.getLocationsPromise(0, 2).then(() => {
      expect(requests.getLocation.mock.calls.length).toBe(2);
      expect(requests.getLocation.mock.calls[0]).toEqual([photos[0].path]);
      expect(requests.getLocation.mock.calls[1]).toEqual([photos[1].path]);
    });
  });

  test('getLocationsPromise calls getLocations for range if filterPhotos is false', () => {
    const pageInstance = page.root.instance;

    pageInstance.setState(
      Object.assign({}, pageInstance.state, { filterPhotos: false })
    );

    return pageInstance.getLocationsPromise(0, 2).then(() => {
      expect(requests.getLocations.mock.calls.length).toBe(1);
      expect(requests.getLocations.mock.calls[0]).toEqual([0, 2]);
    });
  });

  test('getAndStoreLocations sets location, error and loaded photo fields', () => {
    const pageInstance = page.root.instance;

    return pageInstance.getAndStoreLocations(0, 2).then(() => {
      expect(requests.getLocations.mock.calls.length).toBe(1);

      expect(pageInstance.state.photos[0]).not.toBe(photos[0]);
      expect(pageInstance.state.photos[0].location).toBe(undefined);
      expect(pageInstance.state.photos[0].error).toBe('Oh no!');
      expect(pageInstance.state.photos[0].loaded).toBe(true);
      expect(pageInstance.state.photos[1]).not.toBe(photos[1]);
      expect(pageInstance.state.photos[1].location).toEqual({
        Existing: {
          latitude: 5,
          longitude: 30
        }
      });
      expect(pageInstance.state.photos[1].error).toBe(undefined);
      expect(pageInstance.state.photos[1].loaded).toBe(true);
    });
  });
});
