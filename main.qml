import QtQuick 2.6
import QtQuick.Window 2.2
import QtQuick.Controls 1.4
import QtQuick.Dialogs 1.3
import RustCode 1.0


ApplicationWindow {
    visible: true
    width: 640
    height: 480
    title: "Spritesheet Viewer"
    Store {
        id: store
    }
    menuBar: MenuBar {
        Menu {
            title: "File"
            MenuItem {
                text: "Open..."
                onTriggered: {
                    fileDialog.open()
                }
            }

            MenuSeparator {}

            MenuItem {
                text: "Quit"
                onTriggered: {
                    Qt.quit()
                }
            }
        }
    }

    MainForm {
        anchors.fill: parent
    }

    FileDialog {
        id: fileDialog
        title: "Open Sheet"
        folder: shortcuts.home
        nameFilters: [ "JSON files (*.json)", "All files (*)" ]
        selectExisting: true
        selectMultiple: false
        selectFolder: false
        onAccepted: {
            console.log("You chose: " + fileDialog.fileUrls)

            store.guessImagePath(fileDialog.fileUrls);
        }
        onRejected: {
            console.log("Canceled")
        }
    }
}
