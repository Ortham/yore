import * as React from 'react';
import * as renderer from 'react-test-renderer';
import { PhotoThumbnail } from '../../src/gui/js/photo-thumbnail';

describe('PhotoThumbnail', () => {
  let mockHandleSelect: jest.Mock;
  let thumbnail: renderer.ReactTestRendererJSON;

  beforeAll(() => {
    mockHandleSelect = jest.fn();

    thumbnail = renderer
      .create(
        <PhotoThumbnail
          key="path"
          isSelected={false}
          handleSelect={mockHandleSelect}
          photo={{
            path: 'path'
          }}
          loadedClassName="pig-loaded"
          src="/thumbnail?path=path&maxWidth=300&maxHeight=300"
        />
      )
      .toJSON();
  });

  beforeEach(() => {
    mockHandleSelect.mockClear();
  });

  test('renders a div with an image and an icon', () => {
    expect(thumbnail).toMatchSnapshot();
  });

  test('clicking on a PhotoThumbnail should call its handleSelect callback', () => {
    thumbnail.props.onClick();
    expect(mockHandleSelect.mock.calls.length).toBe(1);
  });

  test('a key-up on a PhotoThumbnail should call its handleSelect callback', () => {
    thumbnail.props.onKeyUp();
    expect(mockHandleSelect.mock.calls.length).toBe(1);
  });
});
