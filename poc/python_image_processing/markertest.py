import cv2 as cv
import cv2.aruco as aruco
import numpy as np
import levelcurves


def drawBox(img, points, col = (0, 255, 0)):
    cv.line(img, points[1], points[2], col, 2)
    cv.line(img, points[2], points[3], col, 2)
    cv.line(img, points[0], points[1], col, 2)
    cv.line(img, points[3], points[0], col, 2)


def drawBoxes(img):
    sorted_boxes, boxes_found = findArucoMarkers(img)
    colors = [(255,0,0),(0,255,0),(0,0,255)]  # BGR color space
    for i in range(0,boxes_found):
        drawBox(img, sorted_boxes[i][0], colors[i])


def findArucoMarkers(img):
    color = (255, 10, 255)
    gray = cv.cvtColor(img, cv.COLOR_BGR2GRAY)
    arucoDict = aruco.Dictionary_get(aruco.DICT_6X6_50)
    arucoParam = aruco.DetectorParameters_create()
    bboxs, ids, rejected = aruco.detectMarkers(gray, arucoDict, parameters = arucoParam)

    boxes_found = np.shape(bboxs)[0]
    sorted_boxes = np.zeros((3,1,4,2))
    for i in range(0,boxes_found):
        sorted_boxes[ids[i]] = bboxs[i]

    sorted_boxes = sorted_boxes.astype(int)
    # drawBox(img, sorted_boxes[1][0])

    return sorted_boxes, boxes_found



def findCorners(img):
    sorted_boxes, boxes_found = findArucoMarkers(img)
    if boxes_found == 3:
        average = np.mean(sorted_boxes[0:3:2],(2,0), int)[0]
        # calculate average of blue and red marker

        cv.circle(img, average, 5, (255,255,0),-1)

        distance2 = np.sum((sorted_boxes[2][0] - average) ** 2, axis=1).argmin()
        distance0 = np.sum((sorted_boxes[0][0] - average) ** 2, axis=1).argmin()
        distance1 = np.sum((sorted_boxes[1][0] - average) ** 2, axis=1).argmin()

        cv.circle(img, sorted_boxes[0][0][distance0], 10, (200, 200, 0), -1)
        cv.circle(img, sorted_boxes[1][0][distance1], 10, (200, 200, 0), -1)
        cv.circle(img, sorted_boxes[2][0][distance2], 10, (200, 200, 0), -1)

        closest_corners = np.zeros((4,2), int)
        closest_corners[0] = sorted_boxes[0][0][distance0]
        closest_corners[1] = sorted_boxes[1][0][distance1]
        closest_corners[2] = sorted_boxes[2][0][distance2]

        other_point = sorted_boxes[0][0][(distance0 + 1) % 4]
        other_point_2 = sorted_boxes[2][0][(distance2 - 1) % 4]
        fourth_corner = findFourthPoint(closest_corners[0], other_point, closest_corners[2], other_point_2)

        corners = np.zeros((4, 2), int)
        corners[0] = sorted_boxes[0][0][(distance0 - 1) % 4]
        corners[1] = sorted_boxes[1][0][distance1]
        corners[2] = sorted_boxes[2][0][(distance2 + 1) % 4]

        cv.circle(img, corners[0], 10, (255, 0, 128), -1)
        cv.circle(img, corners[1], 10, (0, 255, 0), -1)
        cv.circle(img, corners[2], 10, (0, 255, 255), -1)

        line_point0 = sorted_boxes[0][0][(distance0 + 2) % 4]
        line_point2 = sorted_boxes[2][0][(distance2 + 2) % 4]
        corners[3] = findFourthPoint(corners[0], line_point0, corners[2], line_point2)

        cv.circle(img, line_point0, 10, (255, 255, 255), -1)
        cv.circle(img, line_point2, 10, (255, 255, 255), -1)
        cv.circle(img, corners[3], 10, (0, 0, 255), -1)

        corners[3] = fourth_corner

        cv.circle(img, other_point, 10, (0, 200, 200), -1)
        cv.circle(img, other_point_2, 10, (0, 200, 200), -1)
        cv.circle(img, fourth_corner, 10, (200, 0, 200), -1)


        for i in range(0,3):
            cv.circle(img, corners[i], 10, (200, 200, 0), -1)
        return corners

    else:
        return None


# finds the intersection of the two lines through the two lines that go through p1, p2 and p3, p4
# points are (x, y)
def findFourthPoint(p1, p2, p3, p4):
    a1, c1 = findLine(p1, p2)
    a2, c2 = findLine(p3, p4)
    if a1 - a2 == 0:
        x = 10000
    else:
        x = (c2 - c1) / (a1 - a2)
    y = a1 * x + c1

    return np.array([x, y], int)


# finds the line through points p1 and p2
def findLine(p1, p2):
    if p1[0] - p2[0] != 0:
        a = (p1[1] - p2[1]) / (p1[0] - p2[0])
    else:
        a = 10000
    c = p1[1] - a * p1[0]
    return a, c


def removePerspective(corners, img):
    image_limits = np.float32([[0,0],[640,0],[640,350],[0,350]])
    corners = np.float32(corners)


    matrix = cv.getPerspectiveTransform(corners, image_limits, )

    result = cv.warpPerspective(img, matrix, (640, 348), None, cv.INTER_CUBIC, cv.BORDER_CONSTANT, 255)
    # 10.5 by 19.3
    return result

def test():
    input1 = cv.imread("images\\photo.png")
    corners = findCorners(input1)

    result = removePerspective(corners, input1)

    cv.imshow("test", result)
    cv.imshow("input", input1)
    cv.waitKey(0)
    cv.destroyAllWindows()


def saveMarkers():
    dictionary = aruco.getPredefinedDictionary(aruco.DICT_6X6_50)

    key = 3
    markerImage = np.zeros((200, 200), dtype=np.uint8)
    aruco.drawMarker(dictionary, key, 200, markerImage, 1)

    cv.imshow("test", markerImage)
    cv.imwrite("images/markers/marker" + str(key) + ".png", markerImage)
    cv.waitKey(0)
    cv.destroyAllWindows()


def showVideo():
    cap = cv.VideoCapture(0)
    cap.set(3, 1000)
    if not cap.isOpened():
        print("Cannot open camera")
        exit()
    while True:
        # Capture frame-by-frame
        ret, frame = cap.read()
        # if frame is read correctly ret is True
        if not ret:
            print("Can't receive frame (stream end?). Exiting ...")
            break
        # Our operations on the frame come here
        corners = findCorners(frame)
        if not (corners is None):
            no_perspective = removePerspective(corners, frame)
            binarized = levelcurves.binarize(no_perspective)
            cv.imshow('noperspective', no_perspective)


        # Display the resulting frame
        cv.imshow('frame', frame)
        if cv.waitKey(1) == ord('q'):
            break
    # When everything done, release the capture
    cap.release()
    cv.destroyAllWindows()


if __name__ == '__main__':
    test()
    # showVideo()

