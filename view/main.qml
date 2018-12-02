import QtQuick 2.6
import QtQuick.Window 2.2
import QtQuick.Controls 1.4
import QtQuick.Dialogs 1.3
import QtQuick.Layouts 1.0
import RustCode 1.0

ApplicationWindow {
    visible: true
    width: 640
    height: 480
    title: "Spritesheet Viewer"
    StoreWrapper {
        id: store
    }

    SplitView {
        anchors.fill: parent
        orientation: Qt.Horizontal

         Rectangle {
            id: viewport
            width: 400
            Layout.minimumWidth: 100
            Layout.fillWidth: true

            color: "lightblue"
            Text {
                text: "View 1"
                anchors.centerIn: parent
            }
        }

        Rectangle {
            id: sideBar
            width: 240
            Layout.minimumWidth: 50
            color: "lightgray"

            Column {
                Label {
                    text: "Sheet Data"
                }
                TextField {
                    readOnly: true
                }
                Button {
                    text: "Browse"
                }
                Label {
                    text: "Atlas"
                }
                TextField {
                    readOnly: true
                }
                Button {
                    text: "Browse"
                }
            }

        }
    }
}
