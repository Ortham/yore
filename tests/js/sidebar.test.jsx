import React from 'react';
import renderer from 'react-test-renderer';
import Sidebar from '../../src/gui/js/sidebar';

const photos = [
  {
    height: 5,
    width: 10,
    loaded: false,
    path: 'path',
    src: 'source'
  },
  {
    height: 5,
    width: 10,
    loaded: true,
    path: 'path',
    src: 'source'
  }
];

let mockGetAndStoreLocations;
let mockHandleFilterToggle;
let mockHandlePhotoSelect;

beforeEach(() => {
  mockGetAndStoreLocations = jest.fn();
  mockHandleFilterToggle = jest.fn();
  mockHandlePhotoSelect = jest.fn();

  mockHandleFilterToggle.mockReturnValueOnce(Promise.resolve());
});

describe('Sidebar', () => {
  test('renders a list of photo thumbnails and a filter checkbox and label', () => {
    const sidebar = renderer
      .create(
        <Sidebar
          photos={photos}
          filterPhotos={false}
          getAndStoreLocations={mockGetAndStoreLocations}
          handleFilterToggle={mockHandleFilterToggle}
          handlePhotoSelect={mockHandlePhotoSelect}
        />
      )
      .toJSON();
    expect(sidebar).toMatchSnapshot();
  });

  test('passing filterPhotos as true should check the filter checkbox', () => {
    const sidebar = renderer
      .create(
        <Sidebar
          photos={photos}
          filterPhotos
          getAndStoreLocations={mockGetAndStoreLocations}
          handleFilterToggle={mockHandleFilterToggle}
          handlePhotoSelect={mockHandlePhotoSelect}
        />
      )
      .toJSON();
    expect(sidebar).toMatchSnapshot();
  });

  test('rowRenderer returns a PhotoThumbnail for the photo at the given index', () => {
    const sidebar = renderer.create(
      <Sidebar
        photos={photos}
        filterPhotos
        getAndStoreLocations={mockGetAndStoreLocations}
        handleFilterToggle={mockHandleFilterToggle}
        handlePhotoSelect={mockHandlePhotoSelect}
      />
    ).root.instance;

    const photoThumbnail = sidebar.rowRenderer({
      index: 0,
      key: 1,
      style: { color: 'black' }
    });
    expect(photoThumbnail).toMatchSnapshot();
  });

  test('rowRender should set the PhotoThumbnail handleSelect to call the handlePhotoSelect callback', () => {
    const sidebar = renderer.create(
      <Sidebar
        photos={photos}
        filterPhotos
        getAndStoreLocations={mockGetAndStoreLocations}
        handleFilterToggle={mockHandleFilterToggle}
        handlePhotoSelect={mockHandlePhotoSelect}
      />
    ).root.instance;

    const photoThumbnail = sidebar.rowRenderer({
      index: 0,
      key: 1,
      style: { color: 'black' }
    });

    photoThumbnail.props.handleSelect();

    expect(mockHandlePhotoSelect.mock.calls.length).toBe(1);
  });

  test('rowHeight scales the height of the photo at the given index to match a width of 272', () => {
    const sidebar = renderer.create(
      <Sidebar
        photos={photos}
        filterPhotos
        getAndStoreLocations={mockGetAndStoreLocations}
        handleFilterToggle={mockHandleFilterToggle}
        handlePhotoSelect={mockHandlePhotoSelect}
      />
    ).root.instance;

    expect(sidebar.rowHeight({ index: 0 })).toBe(136);
  });

  test('isRowLoaded returns false if the photo at the given index has loaded = false', () => {
    const sidebar = renderer.create(
      <Sidebar
        photos={photos}
        filterPhotos
        getAndStoreLocations={mockGetAndStoreLocations}
        handleFilterToggle={mockHandleFilterToggle}
        handlePhotoSelect={mockHandlePhotoSelect}
      />
    ).root.instance;

    expect(sidebar.isRowLoaded({ index: 0 })).toBe(false);
  });

  test('isRowLoaded returns true if the photo at the given index has loaded = true', () => {
    const sidebar = renderer.create(
      <Sidebar
        photos={photos}
        filterPhotos
        getAndStoreLocations={mockGetAndStoreLocations}
        handleFilterToggle={mockHandleFilterToggle}
        handlePhotoSelect={mockHandlePhotoSelect}
      />
    ).root.instance;

    expect(sidebar.isRowLoaded({ index: 1 })).toBe(true);
  });

  test('loadMoreRows should call the getAndStoreLocations callback', () => {
    const sidebar = renderer.create(
      <Sidebar
        photos={photos}
        filterPhotos
        getAndStoreLocations={mockGetAndStoreLocations}
        handleFilterToggle={mockHandleFilterToggle}
        handlePhotoSelect={mockHandlePhotoSelect}
      />
    ).root.instance;

    sidebar.loadMoreRows({ startIndex: 0, stopIndex: 1 });
    expect(mockGetAndStoreLocations.mock.calls.length).toBe(1);
  });

  test('handleFilterToggle should call the handleFilterToggle callback', () => {
    const sidebar = renderer.create(
      <Sidebar
        photos={photos}
        filterPhotos
        getAndStoreLocations={mockGetAndStoreLocations}
        handleFilterToggle={mockHandleFilterToggle}
        handlePhotoSelect={mockHandlePhotoSelect}
      />
    ).root.instance;

    sidebar.list.recomputeRowHeights = jest.fn();

    return sidebar.handleFilterToggle({}).then(() => {
      expect(mockHandleFilterToggle.mock.calls.length).toBe(1);
      expect(sidebar.list.recomputeRowHeights.mock.calls.length).toBe(1);
    });
  });

  test('forceUpdate should also call forceUpdateGrid on the internal list', () => {
    const sidebar = renderer.create(
      <Sidebar
        photos={photos}
        filterPhotos
        getAndStoreLocations={mockGetAndStoreLocations}
        handleFilterToggle={mockHandleFilterToggle}
        handlePhotoSelect={mockHandlePhotoSelect}
      />
    ).root.instance;

    sidebar.list.forceUpdateGrid = jest.fn();

    sidebar.forceUpdate();

    expect(sidebar.list.forceUpdateGrid.mock.calls.length).toBe(1);
  });
});
