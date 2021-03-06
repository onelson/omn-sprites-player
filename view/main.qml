import QtQuick 2.6
import QtQuick.Window 2.2
import QtQuick.Controls 1.4
import QtQuick.Layouts 1.0
import RustCode 1.0


ApplicationWindow {
    visible: true
    width: 640
    height: 480
    title: "Spritesheet Viewer"

    StoreWrapper { id: store }

    FilePicker {
        id: sheetPicker
        nameFilters: [ "JSON files (*.json)", "All files (*)" ]
        onAccepted: {
            console.log("You chose: " + sheetPicker.fileUrls);
            store.sheetPath = sheetPicker.fileUrls.toString();
        }
    }

    FilePicker {
        id: atlasPicker
        nameFilters: [ "PNG files (*.png)", "All files (*)" ]
        onAccepted: {
            console.log("You chose: " + atlasPicker.fileUrls);
            store.imagePath = atlasPicker.fileUrls.toString();
        }
    }

    SplitView {
        anchors.fill: parent
        orientation: Qt.Horizontal

         Rectangle {
            id: viewport
            width: 400
            Layout.minimumWidth: 100
            Layout.fillWidth: true
            color: "darkgrey"

            MouseArea {
                scrollGestureEnabled: false
                anchors.fill: parent
                onWheel: {
                    img.scale += img.scale * wheel.angleDelta.y / 120 / 10;
                }
            }

            Image {
                id: img
                source: "file://" + store.imagePath.toString()
                anchors.centerIn: parent
            }
        }

        Rectangle {
            id: sideBar
            width: 300
            Layout.minimumWidth: 50
            color: "lightgray"

            Column {
                Label { text: "Sheet Data" }
                TextField {
                    readOnly: true
                    text: store.sheetPath
                    width: sideBar.width
                }

                Button {
                    text: "Browse"
                    onClicked: {
                        sheetPicker.open();
                    }
                }
                Label { text: "Atlas" }
                TextField {
                    readOnly: true
                    text: store.imagePath
                    width: sideBar.width
                }
                Button {
                    text: "Browse"
                    onClicked: {
                        atlasPicker.open();
                    }
                }
            }
        }
    }
}
